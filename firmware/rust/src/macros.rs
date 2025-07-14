/// Creates a pin list for the nrf52.
///
/// # Arguments
/// - `peripherals`: The peripheral's instance.
/// - `direct_pins`: A two-dimensional array that represents the physical layout of your keys. Default is for the woagboard
macro_rules! config_matrix_pins {
    (peripherals: $p:ident, direct_pins: [$([$($pin:tt),+ $(,)?]),+ $(,)?]) => {
        {
            #[allow(unused_mut)]
            let mut pins = [
                $(
                    [
                        $(
                            config_matrix_pin_nrf!(@pin $p, $pin)
                        ),+
                    ]
                ),+
            ];
            pins
        }
    };
}

macro_rules! config_matrix_pin_nrf {
    (@pin $p:ident, _) => {
        None
    };

    (@pin $p:ident, $pin:ident) => {
        Some(Input::new($p.$pin, embassy_nrf::gpio::Pull::Up))
    };
}
