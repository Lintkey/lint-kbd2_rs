pub mod channel {
    pub const REPORT_CHANNEL_SIZE: usize = 16;
    pub const KEY_EVENT_CHANNEL_SIZE: usize = 32;
}

pub mod usb {
    // 根据情况填写大小，BUFF_SIZE别写超过片上USB缓存就行
    pub const CFG_DESC_SIZE: usize = 128;
    pub const BOS_DESC_SIZE: usize = 32;
    pub const MSOS_DESC_SIZE: usize = 32;
    pub const USB_BUFF_SIZE: usize = 128;   // f103最大512B
}

pub mod core {
    /// 扫描频率
    pub const SCAN_FREQUENCY: u64 = 10_000;

    /// 消抖判决延迟(ms)，即至少要经过10ms判断出结果
    /// 
    /// 数值越高，按键越不灵敏，但相应的干扰跳动更少
    const DEBOUNCE_THRESHOLD_MS: u32 = 10;

    /// 消抖阈值，不懂不要修改
    pub const DEBOUNCE_THRESHOLD: u16 = ((SCAN_FREQUENCY*(DEBOUNCE_THRESHOLD_MS as u64)) / 1_000) as u16;
}

