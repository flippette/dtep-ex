#![no_std]
#![no_main]

use core::{convert::Infallible, ops::RangeInclusive};

use arduino_hal::{
    adc::AdcSettings, default_serial, entry, pins, Adc, Peripherals,
};
use panic_halt as _;
use ufmt::{uwrite, uwriteln};

const X_FLAT_RANGE: RangeInclusive<u16> = 335..=345;
const Y_FLAT_RANGE: RangeInclusive<u16> = 325..=335;
const Z_FLAT_RANGE: RangeInclusive<u16> = 415..=425;

const X_UPRIGHT_RANGE: RangeInclusive<u16> = 335..=345;
const Y_UPRIGHT_RANGE: RangeInclusive<u16> = 265..=275;
const Z_UPRIGHT_RANGE: RangeInclusive<u16> = 345..=355;

#[entry]
fn _start() -> ! {
    let _ = main();

    #[allow(clippy::empty_loop)]
    loop {}
}

fn main() -> Result<(), Infallible> {
    // SAFETY: we only run this once on init
    let peri = unsafe { Peripherals::steal() };
    let pins = pins!(peri);
    let mut usart = default_serial!(peri, pins, 115_200);

    let mut adc = Adc::new(peri.ADC, AdcSettings::default());
    let x_pin = pins.a0.into_analog_input(&mut adc);
    let y_pin = pins.a1.into_analog_input(&mut adc);
    let z_pin = pins.a2.into_analog_input(&mut adc);

    loop {
        let x = x_pin.analog_read(&mut adc);
        let y = y_pin.analog_read(&mut adc);
        let z = z_pin.analog_read(&mut adc);

        uwrite!(usart, "{} {} {}", x, y, z)?;

        uwriteln!(
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
        )?;
    }
}
