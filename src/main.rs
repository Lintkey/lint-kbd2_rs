#![no_std]
#![no_main]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// 键盘参数配置
mod kbd_cfg;
// 键盘外设
mod kbd_peripherals;
pub(crate) use kbd_peripherals as kbp;
// 运行核心
mod core;
// 键盘key_map
mod key_map;

// logger
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32 as stm32;
use stm32::peripherals;
use stm32::usb;

use kbp::key_scanner::key_state::BitKeyStates;
use core::kbd::debounce::PingPongKeyStates;
use kbp::key_scanner::SPIKeyScanner;

use kbd_cfg::core::*;
use key_map::KEY_NUM;

// 中断向量表
stm32::bind_interrupts!(struct Irqs {
    // 触发usb中断，交给USB Handler处理
    // 这个中断是USB_LP/CAN1_RX0，即低速USB中断/CAN1 RX0中断
    // f103仅支持LP(USB FS, 12Mb/s)，不支持HP(USB HS, 480Mb/s)
    USB_LP_CAN1_RX0 => usb::InterruptHandler<peripherals::USB>;
});

/// 创建外设配置，需要根据硬件配置初始化外设
fn mcu_config() -> stm32::Config {
    let mut config = stm32::Config::default();
    // 配置RCC，不了解的话单看注释很难看懂，建议去找STM32时钟系统框图对着看
    // 主要目标是配置外设时钟和系统时钟，RTC用不到，这里就不配置了
    {
        use stm32::rcc::*;
        let rcc_cfg = &mut config.rcc;
        // 1. 启用高速外部时钟源(HSE)，键盘设计时使用了8Mhz晶振，对应的配置如下
        rcc_cfg.hse = Some(Hse {
            // 时钟频率8MHz
            freq: stm32::time::Hertz(8_000_000),
            // 使用谐振模式(需要mcu控制晶振起振)，另一个模式是直接使用外部时钟信号
            mode: HseMode::Oscillator,
        });
        // 2. 配置锁相环(PLL)进行倍频，将HSE的8MHz倍频到8*9=72MHz(SYSCLK的上限)
        rcc_cfg.pll = Some(Pll {
            src: PllSource::HSE,            // 设置PLL的时钟输入为HSE
            prediv: PllPreDiv::DIV1,        // 不分频
            mul: PllMul::MUL9,              // 9倍频至72MHz
        });
        // 3. 配置其他参数
        // 设置系统时钟源，使用PLL输出的72MHz
        // 注：Cortex系统时钟源是SYSCLK/8，那么MCU的实际频率为72MHz/8=9MHz
        rcc_cfg.sys = Sysclk::PLL1_P;
        // 配置AHB总线时钟分频器，这里选择不分频
        rcc_cfg.ahb_pre = AHBPrescaler::DIV1;
        // 配置APB1外设时钟分频器，注意APB1外设限制了时钟源最大36MHz，这里要2分频
        rcc_cfg.apb1_pre = APBPrescaler::DIV2;
        // APB2外设时钟不分频，因为APB2时钟上限和SYSCLK一致
        rcc_cfg.apb2_pre = APBPrescaler::DIV1;
    }
    config
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // # 初始化外设
    let mcu_peri = {
        #[cfg(not(debug_assertions))] {
            stm32::init(mcu_config())
        }

        #[cfg(debug_assertions)] {
            let mut mcu_peri = stm32::init(mcu_config());

            // 开发模式时USB一直连着电脑，需要device主动下拉dp 10ms提醒电脑重置
            // 后面不用上拉，usb_device第一次run时会自动配置
            #[cfg(debug_assertions)]
            kbp::usb::force_usb_reset(mcu_peri.PA12.reborrow()).await;

            mcu_peri
        }
    };


    // # 创建USB驱动
    // 创建USB驱动builder，先不build，后面要往接口描述符里添加上HID和其他功能
    // 注：stm32f103仅支持从机模式，配置相对简单。其他型号支持OTG，则需要额外配置
    let mut usb_device_builder = {
        let usb_driver = usb::Driver::new(
            // 传入USB外设和中断表配置
            mcu_peri.USB, Irqs,
            // DP，DM脚对应的管脚
            mcu_peri.PA12, mcu_peri.PA11,
        );

        // vid为厂商代号，pid为产品代号，即(vid,pid)对应一个设备
        // vid表可以去USB-IF上翻，随便选一个不出现在上面的数就行
        // 如果电脑识别不到，说明配置的(vid,pid)冲突了，再重新找
        // let usb_cfg = embassy_usb::Config::new(11451,41919);
        let mut usb_cfg = embassy_usb::Config::new(0x63DD,0x0001);
        usb_cfg.manufacturer = Some("lint-kbd");
        usb_cfg.product = Some("lint-kbd2");
        usb_cfg.serial_number = Some("1145149191810");
        usb_cfg.max_power = 450;    // 看你PCB芯片的能耗，国产的74HC165比较垃圾，功耗爆炸
        usb_cfg.supports_remote_wakeup = true;  // 启用远程唤醒

        kbp::usb::get_usb_builder(usb_driver, usb_cfg)
    };

    // 创建KeyboardReport HID ReaderWriter
    // 同时在USB接口描述符里添加该HID的描述符
    // TODO(H): 添加request_handler，处理set_report操作对应的LED
    let (_hid_reader, hid_writer) = kbp::usb::create_hid_reader_writer(&mut usb_device_builder, None);


    // # 创建SPI按键扫描驱动
    type DebounceKeyStates = PingPongKeyStates<KEY_NUM, BitKeyStates<KEY_NUM>, DEBOUNCE_THRESHOLD>;
    let spi_key_device: SPIKeyScanner<'_, DebounceKeyStates, SCAN_FREQUENCY, _> =
        SPIKeyScanner::new_blocking(mcu_peri.SPI2, mcu_peri.PB13, mcu_peri.PB14, mcu_peri.PB15);


    // # 创建键盘核心
    let kbd_core = core::KbdCore::new(key_map::custom_key_map());


    // # 启动
    embassy_futures::join::join3(
        // USB通信
        kbp::usb::run_usb(usb_device_builder.build(), hid_writer),
        // 按键扫描
        spi_key_device.run(),
        // 键盘核心，基于Channel和事件驱动
        kbd_core.run(),
    ).await;
}
