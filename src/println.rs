use crate::{Output, PinA, Serial, _5mA};
use ch58x::ch58x as pac;

static mut SERIAL: Option<Serial<pac::UART3, PinA<5, Output<_5mA>>, PinA<4, Output<_5mA>>>> = None;

pub fn init(serial: Serial<pac::UART3, PinA<5, Output<_5mA>>, PinA<4, Output<_5mA>>>) {
    unsafe { SERIAL = Some(serial) };
}

pub struct Printer;
impl core::fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { SERIAL.as_mut() }
            .ok_or(core::fmt::Error)?
            .write_str(s)
            .map_err(|_| core::fmt::Error)
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            writeln!($crate::println::Printer, $($arg)*).ok();
        }
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            write!($crate::println::Printer, $($arg)*).ok();
        }
    };
}
