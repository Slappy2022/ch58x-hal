#![no_std]
#![no_main]

use ch58x::ch58x as pac;
use ch58x_hal as hal;
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };
    let serial = {
        let uart = peripherals.UART3;
        let tx = hal::PinA::<5>::new().into_output_5ma();
        let rx = hal::PinA::<4>::new().into_output_5ma();
        hal::Serial::new(uart, tx, rx)
    };
    hal::println::init(serial);

    let mut led = hal::PinB::<4>::new().into_output_5ma();

    loop {
        led.toggle();
        hal::delay_ms(1000);

        hal::println!("Hello, World!!");
    }
}
