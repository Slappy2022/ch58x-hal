use ch58x::ch58x as pac;

pub struct OutputPin {
    port: char,
    pin: u8,
}
impl OutputPin {
    pub fn new(port: char, pin: u8) -> Self {
        Self { port, pin }.init()
    }
    fn init(self) -> Self {
        match self.port {
            'A' => unsafe { self.init_a() },
            'B' => unsafe { self.init_b() },
            _ => panic!("port other than A or B"),
        }
        self
    }
    unsafe fn init_a(&self) {
        let pin = self.pin % 8;
        let mask = 1 << pin;
        if self.pin < 8 {
            (*pac::SYS::ptr())
                .pa_dir
                .modify(|r, w| w.pa_dir_0().bits(r.pa_dir_0().bits() | mask));
        } else if self.pin < 16 {
            (*pac::SYS::ptr())
                .pa_dir
                .modify(|r, w| w.pa_dir_1().bits(r.pa_dir_1().bits() | mask));
        } else {
            panic!("Pin larger than 16");
        }
    }
    unsafe fn init_b(&self) {
        let pin = self.pin % 8;
        let mask = 1 << pin;
        if self.pin < 8 {
            (*pac::SYS::ptr())
                .pb_dir
                .modify(|r, w| w.pb_dir_0().bits(r.pb_dir_0().bits() | mask));
        } else if self.pin < 16 {
            (*pac::SYS::ptr())
                .pb_dir
                .modify(|r, w| w.pb_dir_1().bits(r.pb_dir_1().bits() | mask));
        } else if self.pin < 24 {
            (*pac::SYS::ptr())
                .pb_dir
                .modify(|r, w| w.pb_dir_2().bits(r.pb_dir_2().bits() | mask));
        } else {
            panic!("Pin larger than 24");
        }
    }
    pub fn toggle(&self) {
        match self.port {
            'A' => unsafe { self.toggle_a() },
            'B' => unsafe { self.toggle_b() },
            _ => panic!("port other than A or B"),
        }
    }
    unsafe fn toggle_a(&self) {
        let pin = self.pin % 8;
        let mask = 1 << pin;
        if self.pin < 8 {
            (*pac::SYS::ptr())
                .pa_out
                .modify(|r, w| w.pa_out_0().bits(r.pa_out_0().bits() ^ mask));
        } else if self.pin < 16 {
            (*pac::SYS::ptr())
                .pa_out
                .modify(|r, w| w.pa_out_1().bits(r.pa_out_1().bits() ^ mask));
        } else {
            panic!("Pin larger than 16");
        }
    }
    unsafe fn toggle_b(&self) {
        let pin = self.pin % 8;
        let mask = 1 << pin;
        if self.pin < 8 {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.pb_out_0().bits(r.pb_out_0().bits() ^ mask));
        } else if self.pin < 16 {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.pb_out_1().bits(r.pb_out_1().bits() ^ mask));
        } else if self.pin < 24 {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.pb_out_2().bits(r.pb_out_2().bits() ^ mask));
        } else {
            panic!("Pin larger than 24");
        }
    }
    fn set_high_internal(&self) {
        match self.port {
            'A' => unsafe { self.set_high_a() },
            'B' => unsafe { self.set_high_b() },
            _ => panic!("port other than A or B"),
        }
    }
    unsafe fn set_high_a(&self) {
        let pin = self.pin % 8;
        let mask = 1 << pin;
        if self.pin < 8 {
            (*pac::SYS::ptr())
                .pa_out
                .modify(|r, w| w.pa_out_0().bits(r.pa_out_0().bits() | mask));
        } else if self.pin < 16 {
            (*pac::SYS::ptr())
                .pa_out
                .modify(|r, w| w.pa_out_1().bits(r.pa_out_1().bits() | mask));
        } else {
            panic!("Pin larger than 16");
        }
    }
    unsafe fn set_high_b(&self) {
        let pin = self.pin % 8;
        let mask = 1 << pin;
        if self.pin < 8 {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.pb_out_0().bits(r.pb_out_0().bits() | mask));
        } else if self.pin < 16 {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.pb_out_1().bits(r.pb_out_1().bits() | mask));
        } else if self.pin < 24 {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.pb_out_2().bits(r.pb_out_2().bits() | mask));
        } else {
            panic!("Pin larger than 24");
        }
    }
    fn set_low_internal(&self) {
        match self.port {
            'A' => unsafe { self.set_low_a() },
            'B' => unsafe { self.set_low_b() },
            _ => panic!("port other than A or B"),
        }
    }
    unsafe fn set_low_a(&self) {
        let pin = self.pin % 8;
        let mask = 0xFF ^ (1 << pin);
        if self.pin < 8 {
            (*pac::SYS::ptr())
                .pa_out
                .modify(|r, w| w.pa_out_0().bits(r.pa_out_0().bits() & mask));
        } else if self.pin < 16 {
            (*pac::SYS::ptr())
                .pa_out
                .modify(|r, w| w.pa_out_1().bits(r.pa_out_1().bits() & mask));
        } else {
            panic!("Pin larger than 16");
        }
    }
    unsafe fn set_low_b(&self) {
        let pin = self.pin % 8;
        let mask = 0xFF ^ (1 << pin);
        if self.pin < 8 {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.pb_out_0().bits(r.pb_out_0().bits() & mask));
        } else if self.pin < 16 {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.pb_out_1().bits(r.pb_out_1().bits() & mask));
        } else if self.pin < 24 {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.pb_out_2().bits(r.pb_out_2().bits() & mask));
        } else {
            panic!("Pin larger than 24");
        }
    }
}
impl embedded_hal::digital::v2::OutputPin for OutputPin {
    type Error = core::convert::Infallible;
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high_internal();
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low_internal();
        Ok(())
    }
}
