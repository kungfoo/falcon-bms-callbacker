use crate::messages::Message;
use falcon_key_file::FalconKeyfile;
use log::*;
use std::ffi::CString;
use std::io;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;

extern crate user32;
extern crate winapi;

mod keyboard_emulator;

#[derive(Debug)]
pub struct CallbackSender {
    rx: Receiver<Message>,
    key_file: Option<FalconKeyfile>,
}

impl CallbackSender {
    pub async fn new(rx: Receiver<Message>) -> Result<CallbackSender, io::Error> {
        Ok(CallbackSender { rx, key_file: None })
    }

    pub async fn run(mut self) -> Result<(), io::Error> {
        let CallbackSender {
            mut rx,
            key_file: _,
        } = self;

        while let Some(message) = rx.recv().await {
            match message {
                Message::CallbackReceived { callback } => {
                    if let Some(ref kf) = self.key_file {
                        if let Some(callback) = kf.callback(&callback) {
                            info!("Received known callback {:?}", callback);

                            let window_name = CString::new("Falcon BMS").unwrap();

                            unsafe {
                                let window_handle =
                                    user32::FindWindowA(std::ptr::null_mut(), window_name.as_ptr());
                                // probably SetForegroundWindow is enough, it was in the other server code.
                                if window_handle == std::ptr::null_mut() {
                                    error!("Have not found BMS window!");
                                    continue;
                                }
                                user32::SetForegroundWindow(window_handle);
                                user32::ShowWindow(window_handle, 9);
                            }
                            thread::sleep(Duration::from_millis(30));
                            keyboard_emulator::invoke(callback);
                        } else {
                            error!("Received unknown callback named '{}'", callback);
                        }
                    }
                }
                Message::KeyfileRead { key_file } => {
                    debug!("new key file received: {}", key_file.describe());
                    self.key_file = Some(key_file);
                }
            }
        }
        Ok(())
    }
}
