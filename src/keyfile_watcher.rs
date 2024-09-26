use crate::messages::Message;
use bms_sm::StringData;
use bms_sm::StringId;
use log::*;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

pub struct KeyfileWatcher {
    tx: Sender<Message>,
    last_hash: Option<u64>,
}

impl KeyfileWatcher {
    pub async fn new(tx: Sender<Message>) -> Result<KeyfileWatcher, io::Error> {
        Ok(KeyfileWatcher {
            tx,
            last_hash: None,
        })
    }

    pub async fn run(self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let KeyfileWatcher { tx, mut last_hash } = self;

        loop {
            if let Ok(string_data) = StringData::read() {
                debug!("BMS appears to be running...");
                let key_file_path = &string_data[&StringId::KeyFile];

                if key_file_path.len() > 0 {
                    trace!("About to read key file: {:?}", key_file_path);
                    let path = Path::new(key_file_path);
                    let mut file = File::open(&path).unwrap();
                    let file_name = String::from(path.file_name().unwrap().to_str().unwrap());

                    let mut buffer = Vec::new();
                    buffer.clear();
                    let _ = file.read_to_end(&mut buffer);

                    let hash = seahash::hash(&buffer);

                    match last_hash {
                        Some(old_hash) => {
                            if hash != old_hash {
                                debug!("Key file contents changed, re-reading it.");
                                Self::read_key_file_and_send_result(file_name, &file, &tx).await;
                                last_hash = Some(hash);
                            } else {
                                trace!("Keyfile unchanged, hash: {}", hash);
                            }
                        }
                        None => {
                            debug!("Reading key file for the first time.");
                            Self::read_key_file_and_send_result(file_name, &file, &tx).await;
                            last_hash = Some(hash);
                        }
                    }
                }
            }

            sleep(Duration::from_secs(10)).await;
        }
    }

    async fn read_key_file_and_send_result(
        file_name: String,
        mut file: &File,
        tx: &Sender<Message>,
    ) {
        file.rewind().expect("Could not seek to beginning of file.");
        match falcon_key_file::parse(file_name, &file) {
            Ok(key_file) => {
                let message = Message::KeyfileRead { key_file };
                tx.send(message).await.unwrap();
            }
            Err(e) => error!("Could not read key file: {:?}", e),
        }
    }
}
