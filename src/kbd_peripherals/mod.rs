// 键盘外设(Keyboard peripherals)抽象层
// 仅实现USB，按键扫描读取和去抖提供API(方便使用其他扫描方案和去抖方法)
// 通过channel传送按键事件给core处理

pub mod usb;
pub mod key_scanner;
// TODO(L): 添加LED指示灯
pub mod indicator_led;