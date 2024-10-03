#![no_std]
#![no_main]

use arduino_hal::{entry, Peripherals};

mod adxl335;

#[entry]
fn main() -> ! {
    let peri = Peripherals::take().unwrap();
    adxl335::run(peri)
}
