pub mod clk_sys_cfg {
    use ch58x::ch58x as pac;
    pub fn clk_sys_mod() -> u8 {
        unsafe { (*pac::SYS::ptr()).clk_sys_cfg.read().clk_sys_mod().bits() }
    }
    pub fn clk_pll_div() -> u8 {
        unsafe { (*pac::SYS::ptr()).clk_sys_cfg.read().clk_pll_div().bits() }
    }
}
