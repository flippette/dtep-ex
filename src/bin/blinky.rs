#![no_std]
#![no_main]

use arduino_hal::{delay_ms, entry, pins, Peripherals};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peri = Peripherals::take().unwrap();
    let pins = pins!(peri);

    let mut led = pins.d13.into_output();

    loop {
        delay_ms(500);
        led.toggle();
    }
}
