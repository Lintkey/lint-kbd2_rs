use core::sync::atomic::{AtomicBool, Ordering};

use defmt::{error, info};
use embassy_sync::signal::Signal;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_usb::{driver::EndpointError, *};

use crate::core::channel::KEYBOARD_REPORT_CHANNEL;

pub(crate) static USB_CONNECTED: AtomicBool = AtomicBool::new(false);
pub(crate) static NEED_WAKEUP_REMOTE: Signal<ThreadModeRawMutex, ()> = Signal::new();

pub fn set_usb_connected(conneted: bool) {
    USB_CONNECTED.store(conneted, Ordering::Release);
}

pub fn usb_connected() -> bool {
    USB_CONNECTED.load(Ordering::Acquire)
}

pub type _KbdHIDReader<'a, D> = class::hid::HidReader<'a, D, 1>;
pub type KbdHIDWriter<'a, D> = class::hid::HidWriter<'a, D, 8>;

pub async fn run_usb<
    D: driver::Driver<'static>
>(
    mut usb_device: UsbDevice<'static, D>,
    mut kbd_writer: KbdHIDWriter<'static, D>
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

                if let Err(e) = kbd_writer.write_serialize(&report).await {
                    if e != EndpointError::Disabled {
                        error!("Failed to send report: {:?}", e);
                        continue;
                    }

                    NEED_WAKEUP_REMOTE.signal(());
                    // Wait 200ms for the wakeup, then send the report again
                    // Ignore the error for the second send
                    embassy_time::Timer::after_millis(200).await;
                    if let Err(e) = kbd_writer.write_serialize(&report).await {
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