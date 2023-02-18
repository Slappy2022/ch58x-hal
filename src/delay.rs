use embedded_hal::blocking::delay::DelayMs;

pub struct Delay;
impl<T: Into<u32>> DelayMs<T> for Delay {
    fn delay_ms(&mut self, ms: T) {
        crate::delay_ms(ms.into());
    }
}
