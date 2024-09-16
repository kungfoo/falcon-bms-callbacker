use log::*;

use env_logger::Env;

use bms_sm::StringData;
use bms_sm::StringId;
use std::time::Duration;
use tokio::time::sleep;

use std::fs::File;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = Env::default().filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);

    info!("Falcon BMS Callbacker");

    loop {
        let string_data = StringData::read();
        if string_data.is_ok() {
            info!("BMS appears to be running...");
            break;
        }
        sleep(Duration::from_millis(200)).await;
    }

    tokio::spawn(async move {
        loop {
            let string_data = StringData::read().unwrap();
            let key_file = &string_data[&StringId::KeyFile];

            if key_file.len() > 0 {
                debug!("Key file: {:?}", key_file);
                let path = Path::new(key_file);
                let file = File::open(&path).unwrap();
                match falcon_key_file::parse(&file) {
                    Ok(key_file) => {
                        info!("Great success reading the key file!");
                        debug!("{:?}", key_file.callback("SimCursorStopMovement"));
                    },
                    Err(e) => error!("Could not read key file: {:?}", e),
                }
            }

            sleep(Duration::from_secs(5)).await;
        }
    });

    tokio::signal::ctrl_c().await?;
    Ok(())
}
