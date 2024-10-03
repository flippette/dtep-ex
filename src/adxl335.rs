use core::ops::RangeInclusive;

use arduino_hal::{
    adc::AdcSettings, hal::usart::BaudrateArduinoExt, pins, Adc, Peripherals,
    Usart,
};
use panic_halt as _;
use ufmt::{uwrite, uwriteln};

const X_FLAT_RANGE: RangeInclusive<u16> = 335..=345;
const Y_FLAT_RANGE: RangeInclusive<u16> = 325..=335;
const Z_FLAT_RANGE: RangeInclusive<u16> = 415..=425;

const X_UPRIGHT_RANGE: RangeInclusive<u16> = 335..=345;
const Y_UPRIGHT_RANGE: RangeInclusive<u16> = 265..=275;
const Z_UPRIGHT_RANGE: RangeInclusive<u16> = 345..=355;

pub fn run(peri: Peripherals) -> ! {
    let pins = pins!(peri);

    let tx_pin = pins.d1.into_output();
    let rx_pin = pins.d0.into_pull_up_input();
    let mut usart =
        Usart::new(peri.USART0, rx_pin, tx_pin, 9600.into_baudrate());

    let mut adc = Adc::new(peri.ADC, AdcSettings::default());
    let x_pin = pins.a0.into_analog_input(&mut adc);
    let y_pin = pins.a1.into_analog_input(&mut adc);
    let z_pin = pins.a2.into_analog_input(&mut adc);

    loop {
        let x = x_pin.analog_read(&mut adc);
        let y = y_pin.analog_read(&mut adc);
        let z = z_pin.analog_read(&mut adc);

        let _ = uwrite!(usart, "{} {} {}", x, y, z);

        let _ = uwriteln!(
            usart,
            "{}",
            if X_FLAT_RANGE.contains(&x)
                && Y_FLAT_RANGE.contains(&y)
                && Z_FLAT_RANGE.contains(&z)
            {
                " (flat)"
            } else if X_UPRIGHT_RANGE.contains(&x)
                && Y_UPRIGHT_RANGE.contains(&y)
                && Z_UPRIGHT_RANGE.contains(&z)
            {
                " (upright)"
            } else {
                ""
            }
        );
    }
}
