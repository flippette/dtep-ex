#![no_std]
#![no_main]

use arduino_hal::{
    default_serial, delay_ms, entry, pins, prelude::*, Peripherals,
};
use heapless::Vec;
use nb::block;
use panic_halt as _;
use ufmt::uwriteln;

#[entry]
fn _start() -> ! {
    main()
}

fn main() -> ! {
    let peri = Peripherals::take().unwrap();
    let pins = pins!(peri);
    let mut serial = default_serial!(peri, pins, 115_200);
    let mut buf = Vec::<u8, 32>::new();
    let mut led = pins.d13.into_output();
    let mut counter = 0;

    loop {
        buf.clear();
        loop {
            match block!(serial.read()).unwrap() {
                b'\n' => break,
                ch => buf.push(ch).unwrap(),
            }
        }

        if buf.split(u8::is_ascii_whitespace).next() != Some(b"ping") {
            break;
        }

        delay_ms(1_000);

        uwriteln!(serial, "pong {}", counter).unwrap();
        counter += 1;
    }

    led.set_high();
    panic!();
}
