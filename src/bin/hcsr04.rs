#![no_std]
#![no_main]

use arduino_hal::{default_serial, entry, pins, Peripherals};
use panic_halt as _;
use ufmt::uwriteln;

#[entry]
fn _start() -> ! {
    main()
}

fn main() -> ! {
    let peri = Peripherals::take().unwrap();
    let pins = pins!(peri);
    let mut serial = default_serial!(peri, pins, 9600);

    let mut trig = pins.d3.into_output();
    let echo = pins.d2;

    // 1 count per 1/(16e6/64) = 4us
    // reg is 16 bits => overflow after 1/(16e6/64)*2^16 = 262ms
    let timer = peri.TC1;
    timer.tccr1b.write(|w| w.cs1().prescale_64());

    'outer: loop {
        // clear the trig pin
        trig.set_low();
        timer.tcnt1.write(|w| w.bits(0));
        while timer.tcnt1.read().bits() < 1 { /* 4us */ }

        // pulse the trig pin
        trig.set_high();
        timer.tcnt1.write(|w| w.bits(0));
        while timer.tcnt1.read().bits() < 3 { /* 12us */ }
        trig.set_low();

        // wait for first pulse edge
        timer.tcnt1.write(|w| w.bits(0));
        while echo.is_low() {
            if timer.tcnt1.read().bits() >= 50_000 {
                // 50_000 * 4us = 200ms
                uwriteln!(serial, "pulse start timeout").unwrap();
                continue 'outer;
            }
        }
        timer.tcnt1.write(|w| w.bits(0));

        // wait for second pulse edge
        while echo.is_high() {}

        // 4us/count => multipl by 4
        let dur = timer.tcnt1.read().bits().saturating_mul(4);
        let dist = match dur {
            u16::MAX => {
                uwriteln!(serial, "pulse end timeout").unwrap();
                continue 'outer;
            }
            _ => dur / 58,
        };

        uwriteln!(serial, "distance: {} cm", dist).unwrap();

        // wait before pulsing again
        timer.tcnt1.write(|w| w.bits(0));
        while timer.tcnt1.read().bits() < 25_000 { /* 100ms */ }
    }
}
