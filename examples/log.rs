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
    hal::logger::init(log::LevelFilter::Trace);

    let mut led = hal::OutputPin::new('B', 4);
    led.set_high().unwrap();

    loop {
        log::trace!("trace");
        log::debug!("debug");
        log::info!("info");
        log::warn!("warn");
        log::error!("error");
        led.set_high().unwrap();
        hal::delay_ms(200);
        led.set_low().unwrap();
        hal::delay_ms(200);
    }
}
