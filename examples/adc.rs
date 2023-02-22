#![no_std]
#![no_main]

use ch58x::ch58x as pac;
use ch58x_hal as hal;
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    hal::clock::set_sys_clock(hal::clock::ClockSource::Pll60MHz);
    let peripherals = unsafe { pac::Peripherals::steal() };
    let serial = {
        let uart = peripherals.UART3;
        let tx = hal::PinA::<5>::new().into_output_5ma();
        let rx = hal::PinA::<4>::new().into_output_5ma();
        hal::Serial::new(uart, tx, rx)
    };
    hal::println::init(serial);
    hal::logger::init(log::LevelFilter::Trace);
    log::trace!("Logging init");

    let mut led = hal::PinB::<4>::new().into_output_5ma();

    loop {
        log::info!("{:.2}°C", hal::adc::temperature_celsius());
        led.toggle();
        hal::delay_ms(1000);
    }
}
