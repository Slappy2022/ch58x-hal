use ch58x::ch58x as pac;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

static CLOCK_SPEED: u32 = 400_000;

pub trait Instance: core::ops::Deref<Target = pac::i2c::RegisterBlock> {
    fn register_block(&self) -> &pac::i2c::RegisterBlock;
}
impl Instance for pac::I2C {
    fn register_block(&self) -> &pac::i2c::RegisterBlock {
        self
    }
}

pub struct I2c<T, Scl, Sda>
where
    T: Instance,
{
    pub i2c: T,
    pub scl: Scl,
    pub sda: Sda,
}

impl<T, Scl, Sda> I2c<T, Scl, Sda>
where
    T: Instance,
{
    pub fn new(i2c: T, scl: Scl, sda: Sda) -> Self {
        Self { i2c, scl, sda }.init()
    }
    fn ptr(&self) -> &pac::i2c::RegisterBlock {
        self.i2c.register_block()
    }
    fn init(self) -> Self {
        self.ptr().ctrl1.write(|w| w.swrst().bit(true));
        self.ptr().ctrl1.write(|w| w.swrst().bit(false));
        let sys_clock = crate::clock::get_sys_clock();
        self.ptr()
            .ctrl2
            .write(|w| unsafe { w.freq().bits((sys_clock / 1_000_000) as u8) });
        self.ptr().ctrl1.write(|w| w.pe().bit(false));

        self.ptr().ckcfgr.write(|w| {
            unsafe { w.ccr().bits((sys_clock / (CLOCK_SPEED * 25)) as u16) };
            w.duty().bit(true);
            w.f_s().bit(true);
            w
        });
        self.ptr().ctrl1.write(|w| {
            w.pe().bit(true);
            w.smbus().bit(false);
            w.smbtype().bit(false);
            w.ack().bit(true);
            w
        });
        self
    }

    #[inline(always)]
    fn start(&self) {
        //self.ptr().ctrl1.write(|w| w.start().bit(true));
        self.ptr()
            .ctrl1
            .modify(|r, w| unsafe { w.bits(r.bits() | 0x0100) });
        //unsafe { *(0x40004800 as *mut u16) |= 0x0100 };
    }
    #[inline(always)]
    fn stop(&self) {
        //self.ptr().ctrl1.write(|w| w.stop().bit(true));
        //self.ptr() .ctrl1 .modify(|r, w| unsafe { w.bits(r.bits() | 0x0200) });
        unsafe { *(0x40004800 as *mut u16) |= 0x0200 };
    }

    #[inline(always)]
    fn is_addr_set(&self) -> bool {
        let lo = self.ptr().star1.read().bits() as u32;
        let hi = self.ptr().star2().read().bits() as u32;
        let event = hi << 16 | lo;
        event & 0x0000_0002 != 0
    }
}

#[allow(unused)]
impl<T, Scl, Sda> Read for I2c<T, Scl, Sda>
where
    T: Instance,
{
    type Error = core::convert::Infallible;
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.start();
        while !self.ptr().star1.read().sb().bit() {}
        self.ptr()
            .datar
            .write(|w| unsafe { w.datar().bits(address << 1 | 0x01) });
        while !self.is_addr_set() {}
        while self.is_addr_set() {}
        for i in 0..buffer.len() {
            while !self.ptr().star1.read().btf().bit() {}
            buffer[i] = self.ptr().datar.read().datar().bits();
        }
        self.stop();
        Ok(())
    }
}

#[allow(unused)]
impl<T, Scl, Sda> WriteRead for I2c<T, Scl, Sda>
where
    T: Instance,
{
    type Error = core::convert::Infallible;
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write(address, bytes)?;
        self.read(address, buffer)?;
        Ok(())
    }
}

#[allow(unused)]
impl<T, Scl, Sda> Write for I2c<T, Scl, Sda>
where
    T: Instance,
{
    type Error = core::convert::Infallible;
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.start();
        while !self.ptr().star1.read().sb().bit() {}
        self.ptr()
            .datar
            .write(|w| unsafe { w.datar().bits(address << 1) });
        while !self.is_addr_set() {}
        while self.is_addr_set() {}
        for &b in bytes {
            self.ptr().datar.write(|w| unsafe { w.datar().bits(b) });
            while !self.ptr().star1.read().btf().bit() {}
        }
        self.stop();
        Ok(())
    }
}
