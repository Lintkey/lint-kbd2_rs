use crate::kbd::key::{KbdKey, QwertyKey, StateKey};


#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyAction {
    /// 直接触发键
    CK(KbdKey),
    /// 待定键
    UK(UncertKey),
    /// 布局传递用
    TS,
    /// 无动作，注意，KbdKey中禁用了None键，要想表示None需使用该枚举
    None,
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UncertKey {
    SK(StateKey, QwertyKey),
    /// SK with cert time(ms)
    MK(StateKey, QwertyKey, u16)
}