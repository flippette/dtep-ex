#![no_std]
#![no_main]

use core::convert::Infallible;

use arduino_hal::{delay_ms, entry, pins, Peripherals};
use panic_halt as _;

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

    let mut led = pins.d13.into_output();

    loop {
        delay_ms(500);
        led.toggle();
    }
}
