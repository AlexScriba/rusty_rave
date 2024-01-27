// #![cfg_attr(not(test), no_std)]
#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();
    let mut count: i64 = 0;

    loop {
        led.toggle();
        arduino_hal::delay_ms(100);

         ufmt::uwriteln!(&mut serial, "Count is: {}", count).unwrap();
        count += 1;
    }
}
