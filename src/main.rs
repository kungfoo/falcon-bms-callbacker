use log::*;
use std::collections::HashMap;

use config::Config;
use env_logger::Env;
use tokio::sync::mpsc;

mod callback_sender;
mod keyfile_watcher;
mod messages;
mod udp_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::builder()
        .set_default("log_level", "info")?
        .set_default("listen_address", "0.0.0.0")?
        .set_default("listen_port", 9027)?
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let env = Env::default().filter_or("LOG_LEVEL", config.get_string("log_level").unwrap());
    env_logger::init_from_env(env);

    info!("Falcon BMS Callbacker");
    debug!(
        "Config is: {:?}",
        config
            .clone()
            .try_deserialize::<HashMap<String, String>>()?
    );

    let addr = format!(
        "{}:{}",
        config.get_string("listen_address")?,
        config.get_int("listen_port")?
    );

    let (tx, rx) = mpsc::channel::<messages::Message>(32);

    let server = udp_server::Server::new(&addr, tx.clone()).await?;
    tokio::spawn(async move {
        let _ = server.run().await;
    });

    let callback_sender = callback_sender::CallbackSender::new(rx).await?;
    tokio::spawn(async move {
        let _ = callback_sender.run().await;
    });

    let keyfile_watcher = keyfile_watcher::KeyfileWatcher::new(tx.clone()).await?;
    tokio::spawn(async move {
        let _ = keyfile_watcher.run().await;
    });

    tokio::signal::ctrl_c().await?;
    Ok(())
}
