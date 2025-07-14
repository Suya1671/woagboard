#![no_std]
#![no_main]

#[macro_use]
mod macros;

mod common;

use common::{ble_addr, init_adc, mpsl_task, COL, L2CAP_MTU, ROW, SIZE};
use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::gpio::Input;
use embassy_nrf::mode::Async;
use embassy_nrf::peripherals::{RNG, SAADC, USBD};
use embassy_nrf::saadc::{self, Input as _};
use embassy_nrf::{bind_interrupts, rng, usb};
use nrf_mpsl::Flash;
use nrf_sdc::mpsl::MultiprotocolServiceLayer;
use nrf_sdc::{self as sdc, mpsl};
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use rmk::ble::trouble::build_ble_stack;
use rmk::channel::EVENT_CHANNEL;
use rmk::config::StorageConfig;
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::direct_pin::DirectPinMatrix;
use rmk::futures::future::join;
use rmk::input_device::adc::{AnalogEventType, NrfAdc};
use rmk::input_device::battery::ChargingStateReader;
use rmk::split::peripheral::run_rmk_split_peripheral;
use rmk::storage::new_storage_for_split_peripheral;
use rmk::{run_devices, HostResources};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    SAADC => saadc::InterruptHandler;
    RNG => rng::InterruptHandler<RNG>;
    EGU0_SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler, usb::vbus_detect::InterruptHandler;
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
});

/// How many outgoing L2CAP buffers per link
const L2CAP_TXQ: u8 = 3;

/// How many incoming L2CAP buffers per link
const L2CAP_RXQ: u8 = 3;

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<RNG, Async>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new()?
        .support_adv()?
        .support_peripheral()?
        .support_dle_peripheral()?
        .support_phy_update_peripheral()?
        .support_le_2m_phy()?
        .peripheral_count(1)?
        .buffer_cfg(L2CAP_MTU, L2CAP_MTU, L2CAP_TXQ, L2CAP_RXQ)?
        .build(p, rng, mpsl, mem)
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("[WOAG PERIPHERAL] RMK Initialising");
    // Initialize the peripherals and nrf-sdc controller
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3V3);
    nrf_config.dcdc.reg0 = true;
    nrf_config.dcdc.reg1 = true;

    let p = embassy_nrf::init(nrf_config);

    let mpsl_p =
        mpsl::Peripherals::new(p.RTC0, p.TIMER0, p.TEMP, p.PPI_CH19, p.PPI_CH30, p.PPI_CH31);
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    static SESSION_MEM: StaticCell<mpsl::SessionMem<1>> = StaticCell::new();
    let mpsl = MPSL.init(unwrap!(mpsl::MultiprotocolServiceLayer::with_timeslots(
        mpsl_p,
        Irqs,
        lfclk_cfg,
        SESSION_MEM.init(mpsl::SessionMem::new())
    )));
    spawner.must_spawn(mpsl_task(&*mpsl));

    let sdc_p = sdc::Peripherals::new(
        p.PPI_CH17, p.PPI_CH18, p.PPI_CH20, p.PPI_CH21, p.PPI_CH22, p.PPI_CH23, p.PPI_CH24,
        p.PPI_CH25, p.PPI_CH26, p.PPI_CH27, p.PPI_CH28, p.PPI_CH29,
    );
    let mut rng = rng::Rng::new(p.RNG, Irqs);
    let mut rng_generator = ChaCha12Rng::from_rng(&mut rng).unwrap();
    let mut sdc_mem = sdc::Mem::<4616>::new();
    let sdc = unwrap!(build_sdc(sdc_p, &mut rng, mpsl, &mut sdc_mem));

    let mut resources = HostResources::new();
    let stack = build_ble_stack(sdc, ble_addr(), &mut rng_generator, &mut resources).await;

    // Initialize the ADC.
    let adc_pin = saadc::VddhDiv5Input.degrade_saadc();
    // TODO: wait for RMK to allow peripheraL battery charging reporting
    let is_charging_pin = Input::new(p.P0_31, embassy_nrf::gpio::Pull::Up);
    let saadc = init_adc(adc_pin, p.SAADC);
    // Wait for ADC calibration.
    saadc.calibrate().await;

    // let ble_battery_config = BleBatteryConfig::new(Some(is_charging_pin), true, None, false);
    let pins = config_matrix_pins!(
        peripherals: p,
        direct_pins: [
            [P0_03, P0_13, P0_20, P0_07, P0_04],
            [P1_10, P0_28, P0_17, P0_12, P0_08],
            [P1_11, P1_04, P1_02, P0_24, P0_22],
            [P0_09, P1_06, P0_10, _, _]
        ]
    );

    // Initialize flash
    // nRF52840's bootloader starts from 0xF4000(976K)
    let storage_config = StorageConfig {
        start_addr: 0x60000, // 384K
        num_sectors: 32,     // 128K
        ..Default::default()
    };
    let flash = Flash::take(mpsl, p.NVMC);
    let mut storage = new_storage_for_split_peripheral(flash, storage_config).await;

    // Initialize the peripheral matrix
    let debouncer = DefaultDebouncer::<COL, ROW>::new();
    let mut matrix = DirectPinMatrix::<_, _, ROW, COL, SIZE>::new(pins, debouncer, true);
    // let mut matrix = rmk::matrix::TestMatrix::<4, 7>::new();

    let mut charging_state_reader = ChargingStateReader::new(is_charging_pin, true);
    let mut adc_device = NrfAdc::new(
        saadc,
        [AnalogEventType::Battery],
        embassy_time::Duration::from_secs(12),
        None,
    );

    // Start
    join(
        run_devices! (
            (matrix, adc_device, charging_state_reader) => EVENT_CHANNEL, // Peripheral uses EVENT_CHANNEL to send events to central
        ),
        run_rmk_split_peripheral(0, &stack, &mut storage),
    )
    .await;
}
