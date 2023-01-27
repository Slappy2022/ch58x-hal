#![no_std]
#![no_main]

use ch58x_hal as hal;
use panic_halt as _;

use embedded_hal::digital::v2::OutputPin;

use ch58x::ch58x as pac;

#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };
    let serial = {
        let uart = peripherals.UART3;
        let tx = hal::OutputPin::new('A', 5);
        let rx = hal::OutputPin::new('A', 4);
        hal::Serial::new(uart, tx, rx)
    };
    hal::println::init(serial);
    let mut led = hal::OutputPin::new('B', 4);
    led.set_high().unwrap();

    loop {
        let timeout = hal::now_us() + 1_000_000;
        let now = loop {
            let now = hal::now_us();
            if timeout <= now {
                break now;
            }
        };
        hal::println!("timeout diff us: {}", now - timeout);
    }
}
