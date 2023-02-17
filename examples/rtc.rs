#![no_std]
#![no_main]

use ch58x::ch58x as pac;
use ch58x_hal as hal;
use embedded_hal::digital::v2::OutputPin;
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
    hal::logger::init(log::LevelFilter::Trace);
    let mut led = hal::PinB::<4>::new().into_output_5ma();
    led.set_high().unwrap();

    hal::clock::calibrate(hal::clock::CalibrationLevel::Level2048);
    loop {
        let timeout = hal::now_us() + 1_000_000;
        let now = loop {
            let now = hal::now_us();
            if timeout <= now {
                break now;
            }
        };
        log::info!("timeout diff us: {}", now - timeout);
    }
}
