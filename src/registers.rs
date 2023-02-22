pub mod clk_sys_cfg {
    use ch58x::ch58x as pac;
    #[inline(always)]
    pub fn clk_sys_mod() -> u8 {
        unsafe { (*pac::SYS::ptr()).clk_sys_cfg.read().clk_sys_mod().bits() }
    }
    #[inline(always)]
    pub fn clk_pll_div() -> u8 {
        unsafe { (*pac::SYS::ptr()).clk_sys_cfg.read().clk_pll_div().bits() }
    }
}

pub mod hfck_pwr_ctrl {
    pub mod clk_pll_pon {
        use ch58x::ch58x as pac;
        #[inline(always)]
        pub fn read() -> bool {
            unsafe { (*pac::SYS::ptr()).hfck_pwr_ctrl.read().clk_pll_pon().bit() }
        }
        #[inline(always)]
        pub fn write(b: bool) {
            unsafe {
                (*pac::SYS::ptr())
                    .hfck_pwr_ctrl
                    .write(|w| w.clk_pll_pon().bit(b))
            };
        }
    }
}

pub mod pll_config {
    pub mod flash_io_mod {
        use ch58x::ch58x as pac;
        #[inline(always)]
        pub fn write(b: bool) {
            unsafe {
                (*pac::SYS::ptr())
                    .pll_config
                    .write(|w| w.flash_io_mod().bit(b))
            };
        }
    }
}

pub mod flash_cfg {
    #[inline(always)]
    pub fn write(b: u8) {
        unsafe { *(0x40001807 as *mut u8) = b };
    }
}

pub mod osc_cal_cnt {
    #[inline(always)]
    fn read() -> u16 {
        unsafe { *(0x40001050 as *mut u16) }
    }
    pub mod osc_cal_ov_clr {
        static BIT: usize = 14;
        #[inline(always)]
        pub fn write(b: bool) {
            let current = super::read() & !(1u16 << BIT);
            unsafe { *(0x40001050 as *mut u16) = current | (b as u16) << BIT };
        }
    }
    pub mod osc_cal_cnt {
        static MASK: u16 = 0b0001_1111_1111_1111;
        #[inline(always)]
        pub fn read() -> u16 {
            (unsafe { *(0x40001050 as *mut u16) }) & MASK
        }
    }
}

pub mod osc_cal_ov_cnt {
    #[inline(always)]
    pub fn read() -> u8 {
        unsafe { *(0x40001052 as *mut u8) }
    }
}
