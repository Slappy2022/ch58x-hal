#![no_std]
#![no_main]

use ch58x_hal as hal;
use panic_halt as _;
use ch58x::ch58x as pac;

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
    hal::logger::init(log::LevelFilter::Trace);

    let mut led = hal::PinB::<4>::new().into_output_5ma();

    loop {
        log::trace!("trace");
        log::debug!("debug");
        log::info!("info");
        log::warn!("warn");
        log::error!("error");
        led.toggle();
        hal::delay_ms(1000);
    }
}
