use log::*;

use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::io;

pub struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>
}

impl Server {
    pub async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            if let Some((size, peer)) = to_send {
                let amt = socket.send_to(&buf[..size], &peer).await?;
                debug!("Echoed {}/{} bytes to {}", amt, size, peer);
            }

            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }

    pub async fn new(addr: &str) -> Result<Server, io::Error> {
    let socket = UdpSocket::bind(&addr).await?;
    debug!("Listening on: {}", socket.local_addr()?);
        Ok(Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    })
    }
}
