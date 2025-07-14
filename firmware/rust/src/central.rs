#![no_std]
#![no_main]

mod keymap;
#[macro_use]
mod macros;
mod common;
mod vial;

use common::{ble_addr, init_adc, mpsl_task, COL, L2CAP_MTU, ROW, SIZE};
use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Input, Output},
    mode::Async,
    peripherals::{RNG, SAADC, USBD},
    rng,
    saadc::{self, Input as _},
    usb::{self, vbus_detect::HardwareVbusDetect, Driver},
};
use embassy_time::Duration;
use keymap::{get_combos, get_forks, NUM_LAYER};
use nrf_mpsl::Flash;
use nrf_sdc::{
    self as sdc,
    mpsl::{self, MultiprotocolServiceLayer},
};
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use rmk::{
    ble::trouble::build_ble_stack,
    channel::EVENT_CHANNEL,
    config::{
        BehaviorConfig, ControllerConfig, KeyboardUsbConfig, RmkConfig, StorageConfig,
        TapHoldConfig, VialConfig,
    },
    debounce::default_debouncer::DefaultDebouncer,
    futures::future::{join, join4},
    initialize_keymap_and_storage,
    input_device::{
        adc::{AnalogEventType, NrfAdc},
        battery::{BatteryProcessor, ChargingStateReader},
        Runnable,
    },
    keyboard::Keyboard,
    light::LightController,
    run_devices, run_processor_chain, run_rmk,
    split::{
        ble::central::read_peripheral_addresses,
        central::{run_peripheral_manager, CentralDirectPinMatrix},
    },
    HostResources,
};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};
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
const L2CAP_TXQ: u8 = 4;

/// How many incoming L2CAP buffers per link
const L2CAP_RXQ: u8 = 4;

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<RNG, Async>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new()?
        .support_adv()?
        .support_scan()?
        .support_central()?
        .support_peripheral()?
        .support_dle_peripheral()?
        .support_dle_central()?
        .support_phy_update_central()?
        .support_phy_update_peripheral()?
        .support_le_2m_phy()?
        .central_count(1)?
        .peripheral_count(1)?
        .buffer_cfg(L2CAP_MTU, L2CAP_MTU, L2CAP_TXQ, L2CAP_RXQ)?
        .build(p, rng, mpsl, mem)
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("[WOAG CENTRAL] RMK Initialising");
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
    let mut rng_gen = ChaCha12Rng::from_rng(&mut rng).unwrap();
    let mut sdc_mem = sdc::Mem::<8192>::new();
    let sdc = unwrap!(build_sdc(sdc_p, &mut rng, mpsl, &mut sdc_mem));
    let mut host_resources = HostResources::new();
    let stack = build_ble_stack(sdc, ble_addr(), &mut rng_gen, &mut host_resources).await;

    // Initialize usb driver
    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    // Initialize flash
    let flash = Flash::take(mpsl, p.NVMC);

    // Initialize IO Pins
    #[rustfmt::skip]
    let pins = config_matrix_pins!(
        peripherals: p,
        direct_pins: [
            [P0_03, P0_13, P0_20, P0_07, P0_04],
            [P1_10, P0_28, P0_17, P0_12, P0_08],
            [P1_11, P1_04, P1_02, P0_24, P0_22],
            [_, _, P0_09, P1_06, P0_10]
        ]
    );

    // Initialize the ADC.
    let adc_pin = saadc::VddhDiv5Input.degrade_saadc();
    let is_charging_pin = Input::new(p.P0_31, embassy_nrf::gpio::Pull::Up);
    let saadc = init_adc(adc_pin, p.SAADC);
    // Wait for ADC calibration.
    saadc.calibrate().await;

    // Keyboard config
    let keyboard_usb_config = KeyboardUsbConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "Suya Singh",
        product_name: "Woagboard",
        serial_number: "vial:f64c2b3c:000001",
    };
    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF);
    let mut charging_state_reader = ChargingStateReader::new(is_charging_pin, true);
    let storage_config = StorageConfig {
        start_addr: 0xA0000,
        num_sectors: 6,
        ..Default::default()
    };
    let rmk_config = RmkConfig {
        usb_config: keyboard_usb_config,
        vial_config,
        storage_config,
        ..Default::default()
    };

    // Initialze keyboard stuffs
    // Initialize the storage and keymap
    let mut default_keymap = keymap::get_default_keymap();
    let behavior_config = BehaviorConfig {
        tap_hold: TapHoldConfig {
            enable_hrm: true,
            permissive_hold: true,
            chordal_hold: true,
            prior_idle_time: Duration::from_millis(150),
            ..Default::default()
        },
        fork: get_forks(),
        combo: get_combos(),
        ..Default::default()
    };
    let (keymap, mut storage) =
        initialize_keymap_and_storage(&mut default_keymap, flash, &storage_config, behavior_config)
            .await;

    // Initialize the matrix and keyboard
    let debouncer = DefaultDebouncer::<4, 7>::new();
    let mut matrix =
        CentralDirectPinMatrix::<_, _, 0, 0, ROW, COL, SIZE>::new(pins, debouncer, true);
    // let mut matrix = TestMatrix::<ROW, COL>::new();
    let mut keyboard = Keyboard::new(&keymap);

    // Read peripheral address from storage
    let peripheral_addrs =
        read_peripheral_addresses::<1, _, ROW, { COL * 2 }, NUM_LAYER, 0>(&mut storage).await;

    // Initialize the light controller
    let mut light_controller: LightController<Output> =
        LightController::new(ControllerConfig::default().light_config);

    let mut adc_device = NrfAdc::new(
        saadc,
        [AnalogEventType::Battery],
        embassy_time::Duration::from_secs(12),
        None,
    );
    let mut batt_proc = BatteryProcessor::new(2000, 2806, &keymap);

    // Start
    join4(
        run_devices! (
            (matrix, adc_device, charging_state_reader) => EVENT_CHANNEL,
        ),
        run_processor_chain! {
            EVENT_CHANNEL => [batt_proc],
        },
        keyboard.run(),
        join(
            run_peripheral_manager::<ROW, COL, 4, 0, _>(0, peripheral_addrs[0], &stack),
            run_rmk(
                &keymap,
                driver,
                &stack,
                &mut storage,
                &mut light_controller,
                rmk_config,
            ),
        ),
    )
    .await;
}
