#[derive(Clone, Copy, PartialEq)]
pub enum ClockSource {
    Lsi = 0x00,
    Hse16MHz = 0x20 | 2,
    Hse8MHz = 0x20 | 4,
    Hse6_4MHz = 0x20 | 5,
    Hse4MHz = 0x20 | 8,
    Hse2MHz = 0x20 | 16,
    Hse1MHz = 0x20 | 0,
    Pll80MHz = 0x40 | 6,
    Pll60MHz = 0x40 | 8,
    Pll48MHz = 0x40 | 10,
    Pll40MHz = 0x40 | 12,
    Pll36_9MHz = 0x40 | 13,
    Pll32MHz = 0x40 | 15,
    Pll30MHz = 0x40 | 16,
    Pll24MHz = 0x40 | 20,
    Pll20MHz = 0x40 | 24,
    Pll15MHz = 0x40 | 0,
}
impl ClockSource {
    pub fn is_hse(self) -> bool {
        self as u8 & 0x20 != 0
    }
    pub fn is_ppl(self) -> bool {
        self as u8 & 0x40 != 0
    }
    pub fn div(self) -> u8 {
        self as u8 & 0x0f
    }
}

pub fn set_sys_clock(source: ClockSource) {
    use crate::safe::*;
    pll_config::pll_cfg_dat::write(pll_config::pll_cfg_dat::read() & !(1 << 5));
    if source.is_hse() {
        // TODO
    } else if source.is_ppl() {
        if !hfck_pwr_ctrl::clk_pll_pon::read() {
            hfck_pwr_ctrl::clk_pll_pon::write(true);
            unsafe { riscv::asm::delay(4000) };
        }
        clk_sys_cfg::write(Clock::Pll, source.div());
        unsafe { riscv::asm::delay(4) };
        if source == ClockSource::Pll80MHz {
            flash_cfg::write(0x02);
        } else {
            flash_cfg::write(0x52);
        }
    } else {
        // TODO
    }
    pll_config::flash_io_mod::write(true);
}

#[no_mangle]
pub fn get_sys_clock() -> u32 {
    let (clock, div) = crate::safe::clk_sys_cfg::read();
    let div = div as u32;
    use crate::safe::Clock::*;
    match clock {
        Ck32M => 32_000_000 / div,
        Pll => 480_000_000 / div,
        Ck32K => 32_000,
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CalibrationLevel {
    /*
    Level1 = 0b0000_0000,
    Level2 = 0b0000_0001,
    Level4 = 0b0000_0010,
    */
    Level32 = 0b0000_0011,
    Level64 = 0b0000_0100,
    Level128 = 0b0000_0101,
    Level1024 = 0b0000_0110,
    Level2048 = 0b0000_0111,
}
static CAB_LSIFQ: i32 = 32_000;
pub fn calibrate() {
    let calibration_level = CalibrationLevel::Level64;
    use crate::registers::*;
    use crate::safe::*;
    ck32k_config::clk_osc32k_filt::write(true);
    ck32k_config::clk_osc32k_filt::write(false);
    xt32k_tune::xt32k_i_tune::write(0x01);
    osc_cal_ctrl::osc_cnt_en::write(true);
    osc_cal_cnt::osc_cal_ov_clr::write(true);
    while !osc_cal_ctrl::osc_cnt_en::read() {
        osc_cal_ctrl::osc_cnt_en::write(true);
    }
    let freq_sys = get_sys_clock() as i32;
    let mut retry = 0u8;
    loop {
        while !osc_cal_ctrl::osc_cnt_halt::read() {}
        while osc_cal_ctrl::osc_cnt_halt::read() {}
        osc_cal_cnt::osc_cal_ov_clr::write(true);
        while !osc_cal_ctrl::osc_cnt_halt::read() {}
        let osc_cal_cnt = osc_cal_cnt::osc_cal_cnt::read() as i32;
        let osc_cal_ov_cnt = osc_cal_ov_cnt::read() as i32;
        let cnt_offset =
            (osc_cal_cnt & 0x3fff) + osc_cal_ov_cnt * 0x3fff - 2000 * (freq_sys / 1000) / CAB_LSIFQ;
        if ((cnt_offset > -37 * (freq_sys / 1000) / CAB_LSIFQ)
            && (cnt_offset < 37 * (freq_sys / 1000) / CAB_LSIFQ))
            || retry > 2
        {
            if retry > 0 {
                break;
            }
        }
        retry += 1;
        let cnt_offset = match cnt_offset > 0 {
            true => (((cnt_offset * 2) / (74 * (freq_sys / 1000) / 60000)) + 1) / 2,
            false => (((cnt_offset * 2) / (74 * (freq_sys / 1000) / 60000)) - 1) / 2,
        } as i16;
        {
            use crate::safe::int32k_tune::*;
            write(read() + cnt_offset);
        }
    }
    while !osc_cal_ctrl::osc_cnt_halt::read() {}
    osc_cal_cnt::osc_cal_ov_clr::write(true);
    osc_cal_ctrl::osc_cnt_total::write(calibration_level as u8);
    while osc_cal_ctrl::osc_cnt_total::read() != calibration_level as u8 {
        osc_cal_ctrl::osc_cnt_total::write(calibration_level as u8);
    }
    while osc_cal_ctrl::osc_cnt_halt::read() {}
    while !osc_cal_ctrl::osc_cnt_halt::read() {}
    let osc_cal_cnt = osc_cal_cnt::osc_cal_cnt::read() as i32;
    let osc_cal_ov_cnt = osc_cal_ov_cnt::read() as i32;

    let cnt_offset = (osc_cal_cnt & 0x3FFF) + osc_cal_ov_cnt * 0x3FFF
        - 4000 * (1 << calibration_level as u8) * (freq_sys / 1000000) / 256 * 1000
            / (CAB_LSIFQ / 256);
    let cnt_offset = (cnt_offset * 200)
        / (1366 * (((1 << (calibration_level as u8)) / 8) * (freq_sys / 1000)) / 60_000);
    let cnt_offset = match cnt_offset > 0 {
        true => ((cnt_offset + 1) / 2) << 5,
        false => ((cnt_offset + 1) / 2) << 5,
    } as i16;
    {
        use crate::safe::int32k_tune::*;
        write(read() + cnt_offset);
    }
    osc_cal_ctrl::osc_cnt_en::write(false);
}
