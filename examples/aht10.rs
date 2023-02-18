#![no_std]
#![no_main]

use aht10::AHT10;
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
    let i2c = {
        let i2c = peripherals.I2C;
        let scl = hal::PinB::<13>::new().into_pull_up_input();
        let sda = hal::PinB::<12>::new().into_pull_up_input();
        hal::i2c::I2c::new(i2c, scl, sda)
    };
    let mut aht10 = AHT10::new(i2c, hal::delay::Delay).unwrap();
    log::trace!("aht10 init");

    loop {
        let (h, t) = aht10.read().unwrap();
        let celsius = t.celsius();
        let rh = h.rh();
        log::info!("{celsius:.2}°C\t{rh:.2}% rh");
        led.toggle();
        hal::delay_ms(500);
    }
}
