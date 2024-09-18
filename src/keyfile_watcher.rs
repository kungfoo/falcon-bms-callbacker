use crate::messages::Message;
use bms_sm::StringData;
use bms_sm::StringId;
use log::*;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

pub struct KeyfileWatcher {
    tx: Sender<Message>,
}

impl KeyfileWatcher {
    pub async fn new(tx: Sender<Message>) -> Result<KeyfileWatcher, io::Error> {
        Ok(KeyfileWatcher { tx })
    }

    pub async fn run(self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let KeyfileWatcher { tx } = self;

        loop {
            if let Ok(string_data) = StringData::read() {
                info!("BMS appears to be running...");
                let key_file_path = &string_data[&StringId::KeyFile];

                if key_file_path.len() > 0 {
                    debug!("About to read key file: {:?}", key_file_path);
                    let path = Path::new(key_file_path);
                    let file = File::open(&path).unwrap();
                    let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
                    match falcon_key_file::parse(file_name, &file) {
                        Ok(key_file) => {
                            let message = Message::KeyfileRead { key_file };
                            tx.send(message).await.unwrap();
                        }
                        Err(e) => error!("Could not read key file: {:?}", e),
                    }
                }
            }

            sleep(Duration::from_secs(5)).await;
        }
    }
}
