#![no_std]
#![no_main]

// logger
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32 as stm32;
use stm32::peripherals;
use stm32::usb;

// MCU底层配置
mod mcu;
// 键盘lib
mod kbd;
// 键盘外设
mod kbp;
// 运行核心
mod core;
mod constants;

use kbp::key_device::SPIKeyDevice;
use kbp::debounce::PingPongKeyStates;
use crate::core::KeyMap;
use crate::kbd::key_action::KeyAction;

// 中断向量表
stm32::bind_interrupts!(struct Irqs {
    // 触发usb中断，交给USB Handler处理
    // 这个中断是USB_LP/CAN1_RX0，即低速USB中断/CAN1 RX0中断
    // f103仅支持LP(USB FS, 12Mb/s)，不支持HP(USB HS, 480Mb/s)
    USB_LP_CAN1_RX0 => usb::InterruptHandler<peripherals::USB>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // 创建外设配置，根据配置初始化外设
    let peripherals_cfg = {
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
    };
    #[cfg(debug_assertions)]
    let mut mcu_peri = stm32::init(peripherals_cfg);
    #[cfg(not(debug_assertions))]
    let mcu_peri = stm32::init(peripherals_cfg);

    defmt::info!("Successfully initialized peripherals.");

    // 开发模式时USB一直连着电脑，需要device主动下拉dp 10ms提醒电脑重置
    // 后面不用上拉，usb_device第一次run时会自动配置
    #[cfg(debug_assertions)] {
        use stm32::gpio;
        // 按照USB协议，DP拉低10ms通知主机进行复位
        let _dp = gpio::Output::new(
            mcu_peri.PA12.reborrow(),
            gpio::Level::Low,
            gpio::Speed::Low,
        );
        embassy_time::Timer::after_millis(10).await;
    }



    // 创建USB驱动builder，先不build，后面要往接口描述符里添加上HID和其他功能
    // 注：stm32f103仅支持从机模式，配置相对简单。其他型号支持OTG，则需要额外配置
    let mut usb_device_builder = {
        let usb_driver = usb::Driver::new(
            // 传入USB外设和中断表配置?
            mcu_peri.USB, Irqs,
            // DP，DM脚对应的管脚
            mcu_peri.PA12, mcu_peri.PA11,
        );

        // let usb_cfg = embassy_usb::Config::new(11451,41919);
        // vid为厂商代号，pid为产品代号，即(vid,pid)对应一个设备
        // vid表可以去USB-IF上翻，随便选一个不出现在上面的数就行
        // 如果电脑识别不到，说明配置的(vid,pid)冲突了，再重新找
        let mut usb_cfg = embassy_usb::Config::new(0x63DD,0x0001);
        usb_cfg.manufacturer = Some("lint-kbd");
        usb_cfg.product = Some("lint-kbd2");
        usb_cfg.serial_number = Some("1145149191810");
        usb_cfg.max_power = 450;    // 看你PCB芯片的能耗，国产的74HC165比较垃圾，功耗爆炸
        usb_cfg.supports_remote_wakeup = true;  // 启用远程唤醒

        // 根据情况填写大小，BUFF_SIZE别写超过片上USB缓存就行
        const CFG_DESC_SIZE: usize = 128;
        const BOS_DESC_SIZE: usize = 32;
        const MSOS_DESC_SIZE: usize = 32;
        const USB_BUFF_SIZE: usize = 128;   // f103最大512B

        use static_cell::StaticCell;
        static CONFIG_DESC: StaticCell<[u8; CFG_DESC_SIZE]> = StaticCell::new();
        static BOS_DESC: StaticCell<[u8; BOS_DESC_SIZE]> = StaticCell::new();
        static MSOS_DESC: StaticCell<[u8; MSOS_DESC_SIZE]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; USB_BUFF_SIZE]> = StaticCell::new();

        embassy_usb::Builder::new(
            usb_driver,
            usb_cfg,
            &mut CONFIG_DESC.init([0; CFG_DESC_SIZE])[..],
            &mut BOS_DESC.init([0; BOS_DESC_SIZE])[..],
            &mut MSOS_DESC.init([0; MSOS_DESC_SIZE])[..],
            &mut CONTROL_BUF.init([0; USB_BUFF_SIZE])[..],
        )
    };
    // TODO: 添加Handler，以监控USB状态

    // 创建KeyboardReport HID ReaderWriter
    // 同时在USB接口描述符里添加该HID的描述符
    // TODO: 目前使用的报文为通用的6键无冲报文, 后续改为全键无冲描述符
    let kbd_hid_rw = {
        use usbd_hid::descriptor::{SerializedDescriptor, KeyboardReport};
        use embassy_usb::class::hid;
        let kbd_hid_cfg = embassy_usb::class::hid::Config {
            // 报文描述符，用于描述报文格式。键盘报文描述符是固定的，直接调包
            // TODO: 自定义Report，实现全键无冲？
            report_descriptor: KeyboardReport::desc(),
            // TODO: 修改handler，处理set_report报文(led修改报文)，即使没有灯
            request_handler: None,
            // 轮询延迟(ms)，1ms时对应1kHz回报率
            // 印象中高回报率有概率导致CPU卡顿，这里调到500Hz，比显示器刷新率高就行
            poll_ms: 2,
            max_packet_size: 64
        };

        static KBD_HID_STATE: static_cell::StaticCell<hid::State>
            = static_cell::StaticCell::new();

        // 1,8是读写报文的大小，具体内容上网搜
        // KeyboardReport把两个合并到一起变9Byte的结构，但读写时是分开的
        // 或许可以考虑优化？
        hid::HidReaderWriter::<_, 1, 8>::new(
            // 传入builder，方便往接口描述符里添加HID描述符
            &mut usb_device_builder,
            KBD_HID_STATE.init(hid::State::new()),
            kbd_hid_cfg)
    };
    // 考虑到lint-kbd2没键盘灯，这里可以不处理ser_report报文。
    // 翻阅HID手册可知道设备能不处理set_report，
    // 默认request_handler为None时遇到set_report会reject，
    // 对应返回的响应就是STALL，告知主机不支持该操作。
    // TODO: 添加request_handler，处理set_report操作对应的LED
    let (_, kbd_writer) = kbd_hid_rw.split();


    let spi_key_device: SPIKeyDevice<'_, stm32::mode::Blocking, PingPongKeyStates> =
        SPIKeyDevice::new_blocking(mcu_peri.SPI1, mcu_peri.PA5, mcu_peri.PA6, mcu_peri.PA7);


    let kbd_map: KeyMap = [
        [
            KeyAction::None, KeyAction::None, KeyAction::None, KeyAction::None,
            KeyAction::None, KeyAction::None, KeyAction::None, KeyAction::None,
        ],
    ];
    let kbd_core = core::KbdCore::new(kbd_map);



    embassy_futures::join::join3(
        // USB通信
        mcu::usb::run_usb(usb_device_builder.build(), kbd_writer),
        // 按键扫描
        spi_key_device.run(),
        // 键盘核心，基于Channel和事件驱动
        kbd_core.run(),
    ).await;
}