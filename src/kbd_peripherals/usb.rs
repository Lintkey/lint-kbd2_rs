use core::sync::atomic::{AtomicBool, Ordering};

use defmt::{error, info};
use static_cell::StaticCell;
use embassy_stm32 as stm32;
use embassy_sync::signal::Signal;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_usb::{driver::EndpointError, driver::Driver, class::hid, *};

use crate::core::channel::KEYBOARD_REPORT_CHANNEL;


pub(crate) static USB_CONNECTED: AtomicBool = AtomicBool::new(false);
pub(crate) static NEED_WAKEUP_REMOTE: Signal<ThreadModeRawMutex, ()> = Signal::new();

pub fn set_usb_connected(conneted: bool) {
    USB_CONNECTED.store(conneted, Ordering::Release);
}

pub fn usb_connected() -> bool {
    USB_CONNECTED.load(Ordering::Acquire)
}


pub async fn force_usb_reset(usb_dp_pin: stm32::Peri<'_, impl stm32::gpio::Pin>) {
    use stm32::gpio;
    let _dp = gpio::Output::new(usb_dp_pin, gpio::Level::Low, gpio::Speed::Low);
    embassy_time::Timer::after_millis(10).await;
    defmt::info!("USB re-enumerated");
}

pub fn get_usb_builder<
    D: Driver<'static>,
>(
    usb_driver: D, usb_cfg: Config<'static>
) -> Builder<'static, D> {
    use crate::kbd_cfg::usb::*;
    static CONFIG_DESC: StaticCell<[u8; CFG_DESC_SIZE]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; BOS_DESC_SIZE]> = StaticCell::new();
    static MSOS_DESC: StaticCell<[u8; MSOS_DESC_SIZE]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; USB_BUFF_SIZE]> = StaticCell::new();

    Builder::new(
        usb_driver,
        usb_cfg,
        &mut CONFIG_DESC.init([0; CFG_DESC_SIZE])[..],
        &mut BOS_DESC.init([0; BOS_DESC_SIZE])[..],
        &mut MSOS_DESC.init([0; MSOS_DESC_SIZE])[..],
        &mut CONTROL_BUF.init([0; USB_BUFF_SIZE])[..],
    )
}

pub type KbdHIDReader<'a, D> = hid::HidReader<'a, D, 1>;
pub type KbdHIDWriter<'a, D> = hid::HidWriter<'a, D, 8>;

/// 创建HID读写器，需传入usb_builder以将HID实例注册到USB中
pub fn create_hid_reader_writer<D: Driver<'static>>(
    usb_device_builder: &mut Builder<'static, D>,
    request_handler: Option<&'static mut (dyn hid::RequestHandler + 'static)>
) -> (KbdHIDReader<'static, D>, KbdHIDWriter<'static, D>) {
    use usbd_hid::descriptor::{SerializedDescriptor, KeyboardReport};
    let kbd_hid_cfg = hid::Config {
        // 报文描述符，用于描述报文格式。键盘报文描述符是固定的，直接调包
        // TODO(L): 自定义Report，实现全键无冲？
        report_descriptor: KeyboardReport::desc(),
        request_handler: request_handler,
        // 轮询延迟(ms)，1ms时对应1kHz回报率
        poll_ms: 1,
        max_packet_size: 64
    };

    static KBD_HID_STATE: StaticCell<hid::State> = StaticCell::new();

    hid::HidReaderWriter::<_, 1, 8>::new(
        usb_device_builder,
        KBD_HID_STATE.init(hid::State::new()),
        kbd_hid_cfg)
    .split()
}

pub async fn run_usb<
    D: driver::Driver<'static>
>(
    mut usb_device: UsbDevice<'static, D>,
    mut hid_writer: KbdHIDWriter<'static, D>
) {
    loop {
        set_usb_connected(true);

        let usb_device_fut = async {
            usb_device.disable().await;
            loop {
                use embassy_futures::select::{Either, select};

                // usb_device.run()

                usb_device.run_until_suspend().await;
                info!("USB suspend，wating for wakeup.");
                // 设备被挂起了，两种情况下唤醒USB
                match select(usb_device.wait_resume(), NEED_WAKEUP_REMOTE.wait()).await {
                    // 1. remote 主动恢复，resume USB
                    Either::First(_) => info!("USB resume."),
                    // 2. device 唤醒 remote，device发送wakeup报文，等待设备恢复
                    // 交给USB底层实现，直接发包能触发wakeup remote?
                    Either::Second(_) => info!("USB wakeup remote"),
                }
            }
        };

        let kbd_hid_fut = async {
            loop {
                // 获取要发送的报文
                let report = KEYBOARD_REPORT_CHANNEL.receive().await;
                // Only send the report after the connection is established.
                if !usb_connected() {
                    continue;
                };

                if let Err(e) = hid_writer.write_serialize(&report).await {
                    if e != EndpointError::Disabled {
                        error!("Failed to send report: {:?}", e);
                        continue;
                    }

                    NEED_WAKEUP_REMOTE.signal(());
                    // Wait 200ms for the wakeup, then send the report again
                    // Ignore the error for the second send
                    embassy_time::Timer::after_millis(200).await;
                    if let Err(e) = hid_writer.write_serialize(&report).await {
                        error!("Failed to send report after wakeup: {:?}", e);
                    }
                }
            }
        };

        use futures::FutureExt;
        let mut usb_device_task = core::pin::pin!(usb_device_fut.fuse());
        let mut kbd_hid_task = core::pin::pin!(kbd_hid_fut.fuse());

        futures::select_biased! {
            _ = usb_device_task => error!("USB device task has ended"),
            _ = kbd_hid_task => error!("Keyboard HID task has ended"),
        };

        set_usb_connected(false);
    }
}