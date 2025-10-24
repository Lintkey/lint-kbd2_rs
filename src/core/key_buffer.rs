use usbd_hid::descriptor::KeyboardReport;

#[derive(Default)]
pub struct KeyBuffer {
    modifier: u8,
    keycodes: [u8; 6],
    cnt: usize,
}

impl KeyBuffer {
    // TODO: 改成直接生成最终报文
    pub fn get_cur_report(&self) -> KeyboardReport {
        KeyboardReport {
            modifier: self.modifier,
            keycodes: self.keycodes.clone(),
            ..KeyboardReport::default()
        }
    }

    pub fn set_modifier(&mut self, key_code: u8) {
        self.modifier |= 1 << (key_code & 0x0F);
    }

    pub fn unset_modifier(&mut self, key_code: u8) {
        self.modifier &= !(1 << (key_code & 0x0F));
    }

    pub fn presse_key(&mut self, key_code: u8) {
        if self.cnt == 6 {
            defmt::warn!("key_buffer full, can't cache key `{}`", key_code);
            return
        }

        self.keycodes[self.cnt] = key_code;
        self.cnt += 1;
    }

    pub fn release_key(&mut self, key_code: u8) {
        if let Some(index) = self.keycodes.iter().position(|&v| v==key_code) {
            for idx in index..(self.keycodes.len()-1) {
                self.keycodes[idx] = self.keycodes[idx+1];
            }
            self.keycodes[self.cnt] = 0;
            self.cnt -= 1;
        } else {
            defmt::warn!("Release a uncached key `{}` in key_buffer", key_code)
        }
    }
}