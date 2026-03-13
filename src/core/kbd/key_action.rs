use super::key::{KbdKey, QwertyKey, StateKey, LayerKey};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyAction {
    /// 直接触发键
    CK(KbdKey),
    /// 待定键
    UK(UncertKey),
    /// 布局传递用
    TS,
    /// 无动作，注意，KbdKey中禁用了None键，要想表示None需使用该枚举
    NA,
}

impl Default for KeyAction {
    fn default() -> Self {
        Self::NA
    }
}

#[allow(unused)]
pub const NA: KeyAction = KeyAction::NA;
#[allow(unused)]
pub const TS: KeyAction = KeyAction::TS;

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UncertKey {
    SK(StateKey, QwertyKey),
    /// SK with tap threshold
    HK(StateKey, QwertyKey, u16),
}

/// 普通按键逻辑，按下即刻触发
#[allow(unused)]
pub fn ck<K: Into<KbdKey>>(key: K) -> KeyAction {
    KeyAction::CK(key.into())
}

/// 待定键
/// 1. 松开，触发单击，即直接按下QK(QwertyKey)
/// 2. 按住时按了其他键，视为要按下SK(StateKey)+其他键
/// 简单来说就是单击时视为按下QK，按住时再按其他键视为要按下快捷键SK+其他键
#[allow(unused)]
pub fn sk<SK: Into<StateKey>, QK: Into<QwertyKey>>(state_key: SK, qwerty_key: QK) -> KeyAction {
    KeyAction::UK(UncertKey::SK(state_key.into(), qwerty_key.into()))
}

/// 待定键
/// 
/// 逻辑和sk大致相同，区别在于有tap_threshold控制，当按住超过一定时间时
/// 视为不触发轻击逻辑，视为按住StateKey
/// 
/// 这类按键主要用于配合鼠标工作
#[allow(unused)]
pub fn hk<SK: Into<StateKey>, QK: Into<QwertyKey>>(state_key: SK, qwerty_key: QK, tap_threshold_ms: u16) -> KeyAction {
    KeyAction::UK(UncertKey::HK(state_key.into(), qwerty_key.into(), tap_threshold_ms))
}

/// 按住时启用指定层
#[allow(unused)]
pub fn lo(layer: u8) -> KeyAction {
    ck(LayerKey::LayerOn(layer))
}

/// 开关指定层
#[allow(unused)]
pub fn ls(layer: u8) -> KeyAction {
    ck(LayerKey::LayerSwitch(layer))
}


// TODO(L): 用宏代替，方便写布局
#[allow(unused)]
#[macro_export]
macro_rules! k {
    (.) => { $crate::core::kbd::key_action::NA };
}