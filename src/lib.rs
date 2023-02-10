#![no_std]

mod gpio;
pub use gpio::*;

pub mod println;

pub mod logger;

mod rtc;
pub use rtc::*;

mod uart;
pub use uart::*;

pub mod registers;
pub mod sys;

pub fn delay_ms(timeout_ms: u32) {
    // No idea why 15_000 is used here instead of 1_000
    // The "system clock" is probably not what I think it is
    let cycles_per_ms = system_clock_hz() / 15_000;
    unsafe { riscv::asm::delay(cycles_per_ms * timeout_ms) };
}

pub fn system_clock_hz() -> u32 {
    let clock_pre_div = crate::registers::clk_sys_cfg::clk_sys_mod();
    let div = crate::registers::clk_sys_cfg::clk_pll_div() as u32;
    let clock = match clock_pre_div {
        0b01 => 480_000_000 / div,
        0b11 => 32_000,
        _ => 32_000_000 / div,
    };
    clock as u32
}
