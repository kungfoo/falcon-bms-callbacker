use crate::messages::Message;
use log::*;
use tokio::sync::mpsc::Sender;

use std::io;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
    tx: Sender<Message>,
}

impl Server {
    pub async fn new(addr: &str, tx: Sender<Message>) -> Result<Server, io::Error> {
        let socket = UdpSocket::bind(&addr).await?;
        info!("Listening on: {}", socket.local_addr()?);
        Ok(Server {
            socket,
            buf: vec![0; 1024],
            to_send: None,
            tx,
        })
    }

    pub async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
            tx,
        } = self;

        loop {
            if let Some((_size, _peer)) = to_send {
                let data = buf[.._size].to_vec();
                let callback_name = String::from_utf8(data).unwrap().trim().to_string();
                if callback_name.len() > 0 {
                    let message = Message::CallbackReceived {
                        callback: callback_name,
                    };

                    debug!("Received: {:?}", message);
                    tx.send(message).await.unwrap();
                }
            }

            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}
