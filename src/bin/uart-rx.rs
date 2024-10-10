#![no_std]
#![no_main]

use core::convert::Infallible;

use arduino_hal::{
    default_serial, delay_ms, entry, pins, prelude::*, Peripherals,
};
use heapless::Vec;
use nb::block;
use panic_halt as _;
use ufmt::uwriteln;

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
    let mut serial = default_serial!(peri, pins, 115_200);
    let mut buf = Vec::<u8, 32>::new();
    let mut led = pins.d13.into_output();
    let mut counter = 0;

    loop {
        buf.clear();
        loop {
            match block!(serial.read())? {
                b'\n' => break,
                ch => buf.push(ch)?,
            }
        }

        if buf.split(u8::is_ascii_whitespace).next() != Some(b"ping") {
            break;
        }

        delay_ms(1_000);

        uwriteln!(serial, "pong {}", counter)?;
        counter += 1;
    }

    led.set_high();
    panic!();
}
