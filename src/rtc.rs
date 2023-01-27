use ch58x::ch58x as pac;

pub fn rtc_cnt_2s() -> u16 {
    unsafe { (*pac::SYS::ptr()).rtc_cnt_2s.read().bits() }
}

pub fn rtc_cnt_32k() -> u16 {
    unsafe { (*pac::SYS::ptr()).rtc_cnt_32k.read().bits() }
}

pub fn rtc_cnt_day() -> u32 {
    unsafe { (*pac::SYS::ptr()).rtc_cnt_day.read().bits() }
}

pub fn now_us() -> u64 {
    let cnt_32k = rtc_cnt_32k() as u64;
    let cnt_2s = rtc_cnt_2s() as u64;
    let cnt_day = rtc_cnt_day() as u64;
    let count = cnt_day << 32 | cnt_2s << 16 | cnt_32k;
    (1_000_000 * count) / 32_768
}
