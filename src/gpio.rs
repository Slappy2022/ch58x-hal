use ch58x::ch58x as pac;
use core::marker::PhantomData;

pub struct Floating;
pub struct PullDown;
pub struct PullUp;
pub struct _5mA;
pub struct _20mA;

pub struct Input<MODE = Floating>(PhantomData<MODE>);
pub struct Output<MODE = _5mA>(PhantomData<MODE>);

pub struct PinA<const N: u8, MODE = Input<Floating>>(PhantomData<MODE>);
impl<const N: u8, MODE> PinA<N, MODE> {
    pub const fn new() -> PinA<N, Input<Floating>> {
        PinA::<N, Input<Floating>>(PhantomData)
    }
    const fn _new() -> Self {
        Self(PhantomData)
    }
}
impl<const N: u8, MODE> PinA<N, MODE> {
    #[inline(always)]
    fn _set_high(&mut self) {
        unsafe {
            (*pac::SYS::ptr())
                .pa_out
                .modify(|r, w| w.bits(r.bits() | (1 << N)))
        };
    }
    #[inline(always)]
    fn _set_low(&mut self) {
        unsafe {
            (*pac::SYS::ptr())
                .pa_out
                .modify(|r, w| w.bits(r.bits() & !(1 << N)))
        };
    }
    #[inline(always)]
    fn _is_set_low(&self) -> bool {
        unsafe { (*pac::SYS::ptr()).pa_out.read().bits() & 1 << N == 0 }
    }
    #[inline(always)]
    fn _is_low(&self) -> bool {
        unsafe { (*pac::SYS::ptr()).pa_pin.read().bits() & 1 << N == 0 }
    }

    fn mode(&mut self, dir: bool, pu: bool, pd_drv: bool) {
        unsafe {
            (*pac::SYS::ptr())
                .pa_dir
                .modify(|r, w| w.bits(r.bits() & !(1 << N) | (dir as u32) << N));
            (*pac::SYS::ptr())
                .pa_pu
                .modify(|r, w| w.bits(r.bits() & !(1 << N) | (pu as u32) << N));
            (*pac::SYS::ptr())
                .pa_pd_drv
                .modify(|r, w| w.bits(r.bits() & !(1 << N) | (pd_drv as u32) << N));
        }
    }

    pub fn into_floating_input(mut self) -> PinA<N, Input<Floating>> {
        self.mode(false, false, false);
        PinA::_new()
    }
    pub fn into_pull_down_input(mut self) -> PinA<N, Input<PullDown>> {
        self.mode(false, false, true);
        PinA::_new()
    }
    pub fn into_pull_up_input(mut self) -> PinA<N, Input<PullUp>> {
        self.mode(false, true, false);
        PinA::_new()
    }
    pub fn into_output_5ma(mut self) -> PinA<N, Output<_5mA>> {
        self.mode(true, false, false);
        PinA::_new()
    }
    pub fn into_output_20ma(mut self) -> PinA<N, Output<_20mA>> {
        self.mode(true, false, true);
        PinA::_new()
    }
}
impl<const N: u8, MODE> PinA<N, Output<MODE>> {
    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self._is_set_low()
    }
    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        self._is_set_low()
    }
    #[inline(always)]
    pub fn toggle(&mut self) {
        match self._is_set_low() {
            true => self._set_high(),
            false => self._set_low(),
        }
    }
}
impl<const N: u8, MODE> embedded_hal::digital::v2::OutputPin for PinA<N, MODE> {
    type Error = core::convert::Infallible;
    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self._set_high();
        Ok(())
    }
    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self._set_low();
        Ok(())
    }
}

pub struct PinB<const N: u8, MODE = Input<Floating>>(PhantomData<MODE>);
impl<const N: u8, MODE> PinB<N, MODE> {
    pub const fn new() -> PinB<N, Input<Floating>> {
        PinB::<N, Input<Floating>>(PhantomData)
    }
    const fn _new() -> Self {
        Self(PhantomData)
    }
}
impl<const N: u8, MODE> PinB<N, MODE> {
    #[inline(always)]
    fn _set_high(&mut self) {
        unsafe {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.bits(r.bits() | (1 << N)))
        };
    }
    #[inline(always)]
    fn _set_low(&mut self) {
        unsafe {
            (*pac::SYS::ptr())
                .pb_out
                .modify(|r, w| w.bits(r.bits() & !(1 << N)))
        };
    }
    #[inline(always)]
    fn _is_set_low(&self) -> bool {
        unsafe { (*pac::SYS::ptr()).pb_out.read().bits() & 1 << N == 0 }
    }
    #[inline(always)]
    fn _is_low(&self) -> bool {
        unsafe { (*pac::SYS::ptr()).pb_pin.read().bits() & 1 << N == 0 }
    }

    fn mode(&mut self, dir: bool, pu: bool, pd_drv: bool) {
        unsafe {
            (*pac::SYS::ptr())
                .pb_dir
                .modify(|r, w| w.bits(r.bits() & !(1 << N) | (dir as u32) << N));
            (*pac::SYS::ptr())
                .pb_pu
                .modify(|r, w| w.bits(r.bits() & !(1 << N) | (pu as u32) << N));
            (*pac::SYS::ptr())
                .pb_pd_drv
                .modify(|r, w| w.bits(r.bits() & !(1 << N) | (pd_drv as u32) << N));
        }
    }

    pub fn into_floating_input(mut self) -> PinB<N, Input<Floating>> {
        self.mode(false, false, false);
        PinB::_new()
    }
    pub fn into_pull_down_input(mut self) -> PinB<N, Input<PullDown>> {
        self.mode(false, false, true);
        PinB::_new()
    }
    pub fn into_pull_up_input(mut self) -> PinB<N, Input<PullUp>> {
        self.mode(false, true, false);
        PinB::_new()
    }
    pub fn into_output_5ma(mut self) -> PinB<N, Output<_5mA>> {
        self.mode(true, false, false);
        PinB::_new()
    }
    pub fn into_output_20ma(mut self) -> PinB<N, Output<_20mA>> {
        self.mode(true, false, true);
        PinB::_new()
    }
}
impl<const N: u8, MODE> PinB<N, Output<MODE>> {
    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self._is_set_low()
    }
    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        self._is_set_low()
    }
    #[inline(always)]
    pub fn toggle(&mut self) {
        match self._is_set_low() {
            true => self._set_high(),
            false => self._set_low(),
        }
    }
}
impl<const N: u8, MODE> embedded_hal::digital::v2::OutputPin for PinB<N, MODE> {
    type Error = core::convert::Infallible;
    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self._set_high();
        Ok(())
    }
    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self._set_low();
        Ok(())
    }
}
