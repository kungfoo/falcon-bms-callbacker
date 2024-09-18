use log::*;

use env_logger::Env;
use std::env;
use tokio::sync::mpsc;

mod callback_sender;
mod keyfile_watcher;
mod messages;
mod udp_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = Env::default().filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);

    info!("Falcon BMS Callbacker");

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:2727".to_string());

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
