#![no_std]
#![no_main]

use core::{convert::Infallible, iter, str};

use arduino_hal::{default_serial, entry, pins, prelude::*, Peripherals};
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
    let mut cur_id = 0;

    loop {
        // read an entire line, up to 32 bytes
        let msg = iter::repeat_with(|| block!(serial.read()))
            .map_while(Result::ok)
            .take_while(|b| *b != b'\n')
            .take(32)
            .collect::<Vec<_, 32>>();

        let mut parts = msg.split(|b| *b == b' ');

        let Some(kind) = parts
            .next()
            .filter(|s| [&b"send"[..], &b"sync"[..]].contains(s))
        else {
            // invalid request, try resync
            uwriteln!(serial, "sync {}", cur_id)?;
            continue;
        };

        match kind {
            b"send" => {
                uwriteln!(serial, "recv {}", cur_id)?;
                cur_id += 1;
            }
            b"sync" => {
                if let Some(id) = parts
                    .next()
                    .and_then(|s| str::from_utf8(s).ok())
                    .and_then(|s| s.parse::<u32>().ok())
                {
                    cur_id = id;
                }

                uwriteln!(serial, "sync {}", cur_id)?;
            }
            _ => unreachable!(),
        }
    }
}
