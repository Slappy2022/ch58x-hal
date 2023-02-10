macro_rules! safe_write {
    ($in:ident, $addr:literal) => {
        core::arch::asm!(
            "lui {0}, 0x40001",
            "li {1}, 0x57",
            "li {2}, 0xa8",
            "sb {1}, 0x40({0})", // safe on 1
            "sb {2}, 0x40({0})", // safe on 2
            // set register
            concat!("sb {3}, ", stringify!($addr), "({0})"),
            "li {1}, 0x00",
            "sb {1}, 0x40({0})", // safe off
            out(reg) _,  // {0} == base register
            out(reg) _,  // {2} == safe value 1
            out(reg) _,  // {3} == safe value 2
            in(reg) $in, // {4} == target value
        )
    };
}

macro_rules! read_write_bool {
    ($name:ident, $addr:literal, $bit:literal, $type:ty) => {
        pub mod $name {
            #[inline(always)]
            pub fn read() -> bool {
                super::read() & 1 << $bit != 0x00
            }
            #[inline(always)]
            pub fn write(b: bool) {
                let b = (super::read() & !(1 << $bit)) | ((b as $type) << $bit);
                unsafe { safe_write!(b, $addr) }
            }
        }
    };
}

macro_rules! read_write_u8 {
    ($name:ident, $addr:literal, $mask:literal) => {
        pub mod $name {
            #[inline(always)]
            pub fn read() -> u8 {
                super::read() & $mask
            }
            #[inline(always)]
            pub fn write(b: u8) {
                let b = (super::read() & !$mask) | (b & $mask);
                unsafe { safe_write!(b, $addr) }
            }
        }
    };
}

macro_rules! read_write {
    ($name:ident, $addr:literal, $mask:literal, $type:ty) => {
        pub mod $name {
            #[inline(always)]
            pub fn read() -> $type {
                super::read() & $mask
            }
            #[inline(always)]
            pub fn write(b: $type) {
                let b = (super::read() & !$mask) | (b & $mask);
                unsafe { safe_write!(b, $addr) }
            }
        }
    };
}

#[derive(Debug)]
pub enum Clock {
    Ck32M = 0b0000_0000,
    Pll = 0b0100_0000,
    Ck32K = 0b1100_0000,
}

pub mod clk_sys_cfg {
    #[inline(always)]
    pub fn read() -> (super::Clock, u8) {
        let cfg = (unsafe { *(0x40001008 as *mut u16) } & 0xff) as u8;
        let clock = match cfg & 0b1100_0000 {
            0b0100_0000 => super::Clock::Pll,
            0b1100_0000 => super::Clock::Ck32K,
            _ => super::Clock::Ck32M,
        };
        let div = cfg & 0x1f;
        (clock, div)
    }
    #[inline(always)]
    pub fn write(clock: super::Clock, div: u8) {
        let b = clock as u8 | div;
        unsafe { safe_write!(b, 0x08) }
    }
}

pub mod pll_config {
    #[inline(always)]
    pub fn read() -> u8 {
        unsafe { *(0x4000104b as *mut u8) }
    }
    read_write_u8!(pll_cfg_dat, 0x4b, 0b0000_0011);
    read_write_bool!(flash_io_mod, 0x4b, 3, u8);
}

pub mod hfck_pwr_ctrl {
    #[inline(always)]
    pub fn read() -> u8 {
        unsafe { *(0x4000100a as *mut u8) }
    }
    read_write_bool!(clk_pll_pon, 0x0a, 4, u8);
}

pub mod flash_cfg {
    #[inline(always)]
    pub fn write(b: u8) {
        unsafe {
            core::arch::asm!(
                "lui {0}, 0x40001",
                "lui {1}, 0x40002",
                "li {2}, 0x57",
                "li {3}, 0xa8",
                "sb {2}, 0x40({0})",     // safe on 1
                "sb {3}, 0x40({0})",     // safe on 2
                "sb {4}, -2041({1})", // set register
                "li {2}, 0x00",
                "sb {2}, 0x40({0})",     // safe off
                out(reg) _,  // {0} == safe register
                out(reg) _,  // {1} == target register
                out(reg) _,  // {2} == safe value 1
                out(reg) _,  // {3} == safe value 2
                in(reg) b,   // {4} == target value
            )
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Clock32kSource {
    Internal = 0b0000_0010,
    External = 0b0000_0101,
}

pub mod ck32k_config {
    #[inline(always)]
    fn read() -> u8 {
        unsafe { *(0x4000102f as *mut u8) }
    }
    #[inline(always)]
    pub fn clock_32k_source(source: super::Clock32kSource) {
        let source = (read() & !0b0000_0111) | (source as u8);
        unsafe { safe_write!(source, 0x2f) }
    }
    read_write_bool!(clk_osc32k_filt, 0x2f, 3, u8);
}

pub mod xt32k_tune {
    #[inline(always)]
    fn read() -> u8 {
        unsafe { *(0x4000102e as *mut u8) }
    }
    read_write_u8!(xt32k_i_tune, 0x2e, 0b0000_0011);
}

pub mod int32k_tune {
    static MASK: i16 = 0b0001_1111_1111_1111;
    #[inline(always)]
    fn read_internal() -> i16 {
        unsafe { *(0x4000102c as *mut i16) }
    }
    #[inline(always)]
    pub fn read() -> i16 {
        // mask and sign extend
        let read = read_internal() & MASK;
        let read = (read << 3) >> 3;
        read
    }
    #[inline(always)]
    pub fn write(v: i16) {
        let v = (read_internal() & !MASK) | (v & MASK);
        unsafe {
            core::arch::asm!(
                "lui {0}, 0x40001",
                "li {1}, 0x57",
                "li {2}, 0xa8",
                "sb {1}, 0x40({0})", // safe on 1
                "sb {2}, 0x40({0})", // safe on 2
                "sh {3}, 0x2c({0})", // safe on 2
                "li {1}, 0x00",
                "sb {1}, 0x40({0})", // safe off
                out(reg) _,  // {0} == base register
                out(reg) _,  // {2} == safe value 1
                out(reg) _,  // {3} == safe value 2
                in(reg) v,   // {4} == target value
            )
        }
    }
}

pub mod osc_cal_ctrl {
    #[inline(always)]
    fn read() -> u8 {
        unsafe { *(0x40001053 as *mut u8) }
    }
    read_write_bool!(osc_cnt_en, 0x53, 5, u8);
    read_write_bool!(osc_cnt_halt, 0x53, 3, u8);
    read_write!(osc_cnt_total, 0x53, 0b0000_0111, u8);
}
