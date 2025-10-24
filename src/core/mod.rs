// 最顶层抽象，基于事件驱动
// 处理kbd传入的按键事件: input->键位映射->确定按键动作->发送按键报告

// ## 按下与释放
// 

// ## (modify,normal)确定方案
// 首先需要明确的是，待确定键只能有一个。
// 存在待确定键时按下其他按键，触发modify
// 待确定键被释放时，触发normal
// 此外还有一种确定条件：当按下超过判决时间时，触发modify
// 此确定条件是可选的，后续分两种组合键实现

// ## 切层
// 基于计数而非bool状态

// KeyEvent(pos, is_pressed): 按键事件，原始输入
// KeyAction: 按键动作，
// KeyReport: 按键报文，最终输出

/*
O(Fn, Normal)
M(Stat, Normal)
K(all)
 */

pub(crate) mod channel;
pub mod key_buffer;

use crate::constants::{KEY_NUM, LAYER_NUM};
use crate::core::channel::{KEYBOARD_REPORT_CHANNEL, KEY_EVENT_CHANNEL};
use crate::core::key_buffer::KeyBuffer;
use crate::kbd::key::{KbdKey, LayerKey, QwertyKey, StateKey};
use crate::kbd::key_action::{KeyAction, UncertKey};
use crate::kbd::key_event::KeyEvent;

pub type KeyMap = [[KeyAction; KEY_NUM]; LAYER_NUM];

pub struct KbdCore {
    /// 按键报文序列，用于维护按键顺序、构造按键报文
    key_buffer: KeyBuffer,
    /// 待处理的未确定键
    uncert_key: Option<(UncertKey, usize)>,
    /// 键盘按键布局
    key_map: KeyMap,
    /// 键盘Layer激活状态，高层优先级更高
    layer_state: [bool; LAYER_NUM],
    /// 按键动作缓存，用于在松开按键时撤销按键动作
    kbd_cache: [Option<KbdKey>; KEY_NUM],
}

impl KbdCore {
    pub fn new(key_map: KeyMap) -> Self {
        Self {
            key_buffer: KeyBuffer::default(),
            uncert_key: None,
            key_map,
            layer_state: [false; LAYER_NUM],
            kbd_cache: [None; KEY_NUM],
        }
    }

    async fn send_kbd_report(&self) {
        let report = self.key_buffer.get_cur_report();
        KEYBOARD_REPORT_CHANNEL.send(report).await
    }

    pub async fn run(mut self) {
        loop {
            // 有待定键
            if let Some((uncert_key, key_index)) = self.uncert_key.clone() {
                self.process_with_uncert_key(uncert_key, key_index).await;
            } else {
                let event = KEY_EVENT_CHANNEL.receive().await;
                self.process_event(event).await;
            }
        }
    }

    async fn process_with_uncert_key(&mut self, uncert_key: UncertKey, key_index: usize) {
        self.uncert_key = None;
        let mut process_inner = async |state_key: StateKey, qwerty_key: QwertyKey, event: KeyEvent| {
            if event.key_index == (key_index as u8) {
                let kbd_key: KbdKey = qwerty_key.into();
                self.process_press_kbd_key(kbd_key, key_index).await;
                self.process_release_kbd_key(kbd_key, key_index).await;
            } else {
                let kbd_key: KbdKey = state_key.into();
                self.process_press_kbd_key(kbd_key, key_index).await;
                self.process_event(event).await;
            }
        };
        match uncert_key {
            UncertKey::SK(state_key, qwerty_key) => {
                let event = KEY_EVENT_CHANNEL.receive().await;
                process_inner(state_key, qwerty_key, event).await;
            },
            UncertKey::MK(state_key, qwerty_key, time_ms) => {
                let time_ms = embassy_time::Duration::from_micros(time_ms as u64);
                let timeout_fut = embassy_time::with_timeout(time_ms,
                    KEY_EVENT_CHANNEL.receive()
                );
                match timeout_fut.await {
                    Ok(event) => process_inner(state_key, qwerty_key, event).await,
                    Err(_) => self.process_press_kbd_key(state_key.into(), key_index).await,
                }
            },
        }
    }

    async fn process_event(&mut self, event: KeyEvent) {
        let key_index = event.key_index as usize;
        if !event.is_pressed {
            if let Some(kbd_key) = self.kbd_cache[key_index] {
                self.process_release_kbd_key(kbd_key, key_index).await;
                // defmt::info!("{}", size_of::<Option<KbdKey>>());
            }
        } else {
            let action = self.get_press_action(key_index).await;
            self.process_press_action(&action, key_index).await;
        }
    }

    async fn process_press_action(&mut self, action: &KeyAction, key_index: usize) {
        match action {
            KeyAction::CK(kbd_key) => {
                self.process_press_kbd_key(*kbd_key, key_index).await;
            },
            KeyAction::UK(uncert_key) => {
                self.uncert_key = Some((uncert_key.clone(), key_index));
            }
            KeyAction::None => {},
            KeyAction::TS => unreachable!(),
        };
    }

    async fn process_press_kbd_key(&mut self, kbd_key: KbdKey, key_index: usize) {
        match kbd_key {
            KbdKey::Normal(qwerty_key) => {
                self.key_buffer.presse_key(qwerty_key as u8);
                self.send_kbd_report().await;
            },
            KbdKey::State(StateKey::Modifier(modifier_key)) => {
                self.key_buffer.set_modifier(modifier_key as u8);
                self.send_kbd_report().await;
            },
            KbdKey::State(StateKey::Layer(layer_key)) => {
                match layer_key {
                    LayerKey::LayerOn(layer) => {
                        self.layer_state[layer as usize] = true;
                    },
                    LayerKey::LayerSwitch(layer) => {
                        self.layer_state[layer as usize] = true;
                    },
                }
            },
        }
        self.kbd_cache[key_index] = Some(kbd_key);
    }

    async fn process_release_kbd_key(&mut self, kbd_key: KbdKey, key_index: usize) {
        match kbd_key {
            KbdKey::Normal(qwerty_key) => {
                self.key_buffer.release_key(qwerty_key as u8);
                self.send_kbd_report().await;
            }
            KbdKey::State(StateKey::Modifier(modifier_key)) => {
                self.key_buffer.unset_modifier(modifier_key as u8);
                self.send_kbd_report().await;
            },
            KbdKey::State(StateKey::Layer(layer_key)) => {
                match layer_key {
                    LayerKey::LayerOn(layer) => {
                        self.layer_state[layer as usize] = false;
                    },
                    LayerKey::LayerSwitch(_layer) => {
                    },
                }
            },
        }
        self.kbd_cache[key_index] = None;
    }

    async fn get_press_action(&self, key_index: usize) -> KeyAction {
        for (layer_idx, key_map) in self.key_map.iter().enumerate().rev() {
            if self.layer_state[layer_idx] {
                let action = &key_map[key_index];
                if *action == KeyAction::TS {
                    continue;
                }

                return action.clone();
            }
        }
        KeyAction::None
    }
}

