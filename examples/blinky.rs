#![no_std]
#![no_main]

use ch58x_hal as hal;
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let mut led = hal::PinB::<4>::new().into_output_5ma();

    loop {
        led.toggle();
        hal::delay_ms(200);
    }
}
