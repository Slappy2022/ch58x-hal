use ch58x::ch58x as pac;

pub fn ticks() -> u64 {
    let ticks_hi = unsafe { (*pac::SYSTICK::ptr()).cnth.read().bits() } as u64;
    let ticks_lo = unsafe { (*pac::SYSTICK::ptr()).cntl.read().bits() } as u64;
    ticks_hi << 32 | ticks_lo
}
