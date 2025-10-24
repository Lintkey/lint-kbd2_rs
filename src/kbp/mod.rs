// 键盘外设(Keyboard peripherals)抽象层
// 仅实现按键扫描读取和去抖
// 通过channel传送按键事件给core处理

pub mod key_state;
pub mod key_device;
pub mod debounce;