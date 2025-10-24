use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use usbd_hid::descriptor::KeyboardReport;

use crate::constants::{KEY_EVENT_CHANNEL_SIZE, REPORT_CHANNEL_SIZE};
use crate::kbd::key_event::KeyEvent;

/// 按键事件
pub static KEY_EVENT_CHANNEL: Channel<ThreadModeRawMutex, KeyEvent, KEY_EVENT_CHANNEL_SIZE> = Channel::new();
/// 按键报告事件
pub static KEYBOARD_REPORT_CHANNEL: Channel<ThreadModeRawMutex, KeyboardReport, REPORT_CHANNEL_SIZE> = Channel::new();