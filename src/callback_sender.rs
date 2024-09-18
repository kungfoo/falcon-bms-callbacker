use crate::messages::Message;
use falcon_key_file::FalconKeyfile;
use log::*;
use std::io;
use tokio::sync::mpsc::Receiver;

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
                    debug!("callback received: {}", callback);
                    if let Some(ref kf) = self.key_file {
                        if let Some(callback) = kf.callback(&callback) {
                            info!("Received known callback {:?}", callback);
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
