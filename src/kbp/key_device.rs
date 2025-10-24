use defmt::error;
use embassy_stm32 as stm32;
use stm32::gpio;
use stm32::spi;

use crate::constants::{KEY_NUM, SCAN_FREQUENCY};
use crate::core::channel::KEY_EVENT_CHANNEL;
use crate::kbd::key_event::KeyEvent;
use crate::kbp::debounce::{DebounceKeyStates, KeyDiffTrait};
use crate::kbp::key_state::BitKeyStates;

/// 基于74H165的按键扫描方案
pub(crate) struct SPIKeyDevice<
    'd,
    SPIMode: stm32::mode::Mode,
    DStates: DebounceKeyStates
> {
    spi_key: spi::Spi<'d, SPIMode>,
    plen: gpio::Output<'d>,
    key_states: DStates,
}

impl<'d, DStates: DebounceKeyStates> SPIKeyDevice<'d, stm32::mode::Blocking, DStates> {
    pub fn new_blocking<T: spi::Instance>(
        peri: stm32::Peri<'d, T>,
        sclk: stm32::Peri<'d, impl spi::SckPin<T>>,
        miso: stm32::Peri<'d, impl spi::MisoPin<T>>,
        plen: stm32::Peri<'d, impl gpio::Pin>,
    ) -> Self {
        let spi_key = {
            let mut spi_cfg = spi::Config::default();
            // 74HC165是上升沿时进行shift操作
            // MODE_1为CPOL=0 CPHA=1
            // 即周期开始时为上升沿并输出，周期中间为下降沿并采样
            spi_cfg.mode = spi::MODE_1;
            // SW1在第一位，所以用LSB
            spi_cfg.bit_order = spi::BitOrder::LsbFirst;
            // 扫描速度1MHz，扫描轮询速度设为10KHz，即扫描间隔100us
            // 消抖设定为10ms
            spi_cfg.frequency = stm32::time::Hertz(1_000_000);
            spi_cfg.gpio_speed = gpio::Speed::VeryHigh;
            // stm32 miso口没有外置或内置上/下拉电阻，使用推挽模式
            spi_cfg.miso_pull = gpio::Pull::None;

            spi::Spi::new_blocking_rxonly(peri, sclk, miso, spi_cfg)
        };

        SPIKeyDevice {
            spi_key,
            plen: gpio::Output::new(plen, gpio::Level::High, gpio::Speed::VeryHigh),
            key_states: DStates::default()
        }
    }

    async fn parallel_load(&mut self) {
        use embassy_time::*;
        self.plen.set_low();
        Timer::after_micros(2).await;
        self.plen.set_high();
        Timer::after_micros(2).await;
    }

    /// `scan_freq` 用于控制扫描频率，设置为回报率的倍数即可
    /// 
    /// 注意，`scan_freq`
    pub(crate) async fn run(mut self) {
        use embassy_time::*;
        let scan_interval = Duration::from_hz(SCAN_FREQUENCY as u64);
        let mut ticker = Ticker::every(scan_interval);

        // 重置plen
        self.plen.set_high();
        Timer::after_micros(2).await;

        loop {
            self.parallel_load().await;

            let mut read_buf = BitKeyStates::default();
            if let Err(e) = self.spi_key.blocking_read(&mut read_buf) {
                error!("Failed to scan keyboard via SPI: {:?}", e);
            }
            let diff = self.key_states.debounce(&read_buf);

            for index in 0..KEY_NUM {
                if diff.is_different(index) {
                    let key_event = {
                        let is_pressed = self.key_states.is_pressed(index);
                        let index = index as u8;
                        KeyEvent::new(is_pressed, index)
                    };
                    KEY_EVENT_CHANNEL.send(key_event).await;
                }
            }

            ticker.next().await
        }
    }
}