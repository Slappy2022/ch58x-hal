#![no_std]
#![no_main]

use ch58x_hal as hal;
use panic_halt as _;

use embedded_hal::digital::v2::OutputPin;

#[riscv_rt::entry]
fn main() -> ! {
    let mut led = hal::OutputPin::new('B', 4);

    loop {
        led.set_high().unwrap();
        hal::delay_ms(200);
        led.set_low().unwrap();
        hal::delay_ms(200);
    }
}
