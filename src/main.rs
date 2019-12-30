#![no_std]
#![no_main]

extern crate panic_halt;
extern crate microbit;

use cortex_m_rt::entry;

extern crate cortex_m_semihosting as sh;
use core::fmt::Write;
use sh::hio;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;
    let mut stdout = hio::hstdout().unwrap();
    stdout.write_str("semitest\n\r").unwrap();
    // or
    writeln!(hio::hstdout().unwrap(), "Init").unwrap();
    loop {}
}
