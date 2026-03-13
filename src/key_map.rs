use crate::core::kbd::key_action::KeyAction;

/// 按键数量
pub const KEY_NUM: usize = 55;
// 注意虽然lint-kbd设计上只接了54个按键，但是要读取55个bit(因为第一位没接按键)，所以应设置为55

/// 按键层数
pub const LAYER_NUM: usize = 4;

pub type KeyMap = super::core::KeyMap<KEY_NUM, LAYER_NUM>;
pub type LogicalIndices = [usize; KEY_NUM];

// 逻辑位置到物理连线的映射(注意EDA上的元件标号是从1开始的，放这里需要改成从0开始)
const PHYSICAL_INDICES: LogicalIndices = [
//  ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,   ,
    6,  5,  4,  1,  2,  3,
    7,  11, 10, 12, 8,  9,      29, 33, 36, 41, 51, 54,
    13, 15, 17, 21, 27, 24, 30, 34, 32, 37, 40, 50, 53,
    14, 19, 16, 22, 26, 28,     38, 43, 44, 46, 49, 52,
        18, 20, 23, 25, 31, 35, 39, 42, 45, 47, 48,
    0,
];

pub fn custom_key_map() -> KeyMap {
    let mut key_map = default_key_map();

    use super::core::kbd::key_action::*;
    use super::core::kbd::key::basic_key::*;

    key_map[0][3] = ck(A);
    key_map[0][4] = ck(B);

    physical_map(key_map)
}

pub fn default_key_map() -> KeyMap {
    [[KeyAction::NA; _]; _]
}

// 映射转换
pub fn physical_map(key_map: KeyMap) -> KeyMap {
    let mut mapped_key_map = default_key_map();
    
    for (unmapped_layer, mapped_layer) in key_map.iter().zip(mapped_key_map.iter_mut()) {
        for (&key_action, physical_index) in core::iter::zip(unmapped_layer, PHYSICAL_INDICES) {
            mapped_layer[physical_index] = key_action;
        }
    }

    mapped_key_map
}