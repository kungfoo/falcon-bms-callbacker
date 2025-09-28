use figment::providers::Format;
use crate::config::Config;
use figment::{providers::{Serialized, Toml}, Figment};
use log::*;

use env_logger::Env;
use tokio::sync::mpsc;

mod callback_sender;
mod keyfile_watcher;
mod messages;
mod udp_server;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config: Config = Figment::new()
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file("config.toml"))
        .extract()
        .expect("Failed to parse config.");

    let env = Env::default().filter_or("LOG_LEVEL", config.log_level.clone());
    env_logger::init_from_env(env);

    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or("Could not determine version!");

    info!("Falcon BMS Callbacker version: {}", version);
    debug!("Config is: {:?}",&config);

    let addr = format!(
        "{}:{}",
        config.listen_address,
        config.listen_port
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
