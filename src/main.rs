#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate microbit;

use core::fmt::Write;
use rt::entry;
use sh::hio;

use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Start").unwrap();
    if let Some(p) = microbit::Peripherals::take() {
        // Split GPIO
        let mut gpio = p.GPIO.split();
        // Configure RX and TX pins accordingly
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        // Configure serial communication
        let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
        write!(tx, "serial - start\r\n");
        // Get row and column for display
        let mut led = gpio.pin13.into_push_pull_output();
        let _ = gpio.pin4.into_push_pull_output();
        // Set row high (column starts low)
        led.set_high();
        // Write string with newline and carriage return
        write!(tx, "serial - LED on\r\n");
    }
    panic!("End");
}
