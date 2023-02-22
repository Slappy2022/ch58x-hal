use ch58x::ch58x as pac;

#[inline(always)]
fn rom_config_temperature_25c() -> (i32, i32) {
    let value = unsafe { *(0x0007f014 as *mut u32) };
    let ref_temp = match value >> 16 {
        0 => 25,
        x => x,
    };
    let ref_raw = value & 0xffff;
    (ref_temp as i32, ref_raw as i32)
}

pub fn temperature_celsius() -> f32 {
    unsafe {
        (*pac::SYS::ptr())
            .tkey_cfg
            .write(|w| w.tkey_pwr_on().bit(false));
        (*pac::SYS::ptr())
            .tem_sensor
            .write(|w| w.tem_sen_pwr_on().bit(true));
        (*pac::SYS::ptr())
            .adc_channel
            .write(|w| w.adc_ch_inx().bits(0x0f));
        (*pac::SYS::ptr()).adc_cfg.write(|w| {
            w.adc_clk_div().bits(0b00);
            w.adc_pga_gain().bits(0b11);
            w.adc_ofs_test().bit(false);
            w.adc_diff_en().bit(true);
            w.adc_buf_en().bit(false);
            w.adc_power_on().bit(true);
            w
        });
        (*pac::SYS::ptr())
            .adc_convert
            .write(|w| w.adc_start().bit(true));
        while (*pac::SYS::ptr()).adc_convert.read().adc_start().bit() {}
        let _ = (*pac::SYS::ptr()).adc_data.read().bits() as i32;

        let mut sum = 0;
        for _i in 0..16 {
            sum += (*pac::SYS::ptr()).adc_data.read().bits() as i32;
        }
        let raw_temp = sum / 16;
        let (ref_temp, ref_raw) = rom_config_temperature_25c();
        let result = ref_temp * 100 + ((raw_temp - ref_raw) * 1000) / 27;
        (result as f32) / 100.0
    }
}
