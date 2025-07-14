use defmt::unwrap;
use embassy_nrf::{
    interrupt::{self, InterruptExt},
    saadc::{self, AnyInput, Input, Saadc},
    Peri,
};
use nrf_mpsl::MultiprotocolServiceLayer;

use crate::{Irqs, SAADC};

pub(crate) const COL: usize = 5;
pub(crate) const ROW: usize = 4;
pub(crate) const SIZE: usize = 32;

/// Size of L2CAP packets
pub const L2CAP_MTU: u16 = 251;

#[embassy_executor::task]
pub async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    mpsl.run().await
}

/// Initializes the SAADC peripheral in single-ended mode on the given pin.
pub fn init_adc(adc_pin: AnyInput, adc: Peri<'static, SAADC>) -> Saadc<'static, 1> {
    // Then we initialize the ADC. We are only using one channel in this example.
    let config = saadc::Config::default();
    let channel_cfg = saadc::ChannelConfig::single_ended(adc_pin.degrade_saadc());
    interrupt::SAADC.set_priority(interrupt::Priority::P3);
    let saadc = saadc::Saadc::new(adc, Irqs, config, [channel_cfg]);
    saadc
}

pub fn ble_addr() -> [u8; 6] {
    let ficr = embassy_nrf::pac::FICR;
    let high = u64::from(ficr.deviceid(1).read());
    let addr = high << 32 | u64::from(ficr.deviceid(0).read());
    let addr = addr | 0x0000_c000_0000_0000;
    unwrap!(addr.to_le_bytes()[..6].try_into())
}
