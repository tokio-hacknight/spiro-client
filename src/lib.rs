extern crate tokio_core;

use std::net::{SocketAddr, ToSocketAddrs};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

pub struct Client {
    core: Core,
    socket: UdpSocket,
    socket_addr: SocketAddr,
}

impl Client {
    pub fn new(socket_addr: &str) -> std::io::Result<Self> {
        let port: u16 = 26262;
        let core = try!(Core::new());
        let handle = core.handle();
        let socket_addr: SocketAddr = try!((socket_addr, port).to_socket_addrs()).next().unwrap();
        let socket = UdpSocket::bind(&socket_addr, &handle).unwrap();
        Ok(Client { core: core, socket: socket, socket_addr: socket_addr })
    }

    pub fn send_params(&mut self, x: f64, y: f64, p1: f64, p2: f64) -> std::io::Result<usize> {
        let data = format!("{}\t{}\t{}\t{}", x, y, p1, p2);
        self.socket.send_to(data.as_bytes(), &self.socket_addr)
    }
}

// extern "C" fn new_client() -> *mut Client {
//      let client = Box::new(Client::new("127.0.0.1"));
//      unsafe {
//          &mut *client
//      }
// }

pub extern "C" fn send_params(client: *mut Client, x: f64, y: f64, p1: f64, p2: f64) {
    unsafe {
        let _ = (*client).send_params(x, y, p1, p2);
    }
}