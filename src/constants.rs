// --- 以下为可配置项 ---
pub const REPORT_CHANNEL_SIZE: usize = 16usize;
pub const KEY_EVENT_CHANNEL_SIZE: usize = 32usize;
// pub const KEY_NUM: usize = 56usize;
pub const KEY_NUM: usize = 8usize;
// pub const LAYER_NUM: usize = 4usize;
pub const LAYER_NUM: usize = 1usize;
/// 扫描频率
pub const SCAN_FREQUENCY: u64 = 10_000;
// 消抖判决延迟(ms)，即至少要经过10ms判断出结果
// 数值越高，按键越不灵敏，但相应的干扰跳动更少
pub const DEBOUNCE_THRESHOLD_MS: u32 = 10;

// --- 以下为自动计算项 ---
pub const KEY_BYTES_LEN: usize = (KEY_NUM+7usize)/8usize;
pub const DEBOUNCE_THRESHOLD: u16 = ((SCAN_FREQUENCY*1_000)/(DEBOUNCE_THRESHOLD_MS as u64)) as u16;