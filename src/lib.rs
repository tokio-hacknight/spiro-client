extern crate libc;
extern crate futures;
#[macro_use]
extern crate tokio_core;

use libc::c_char;
use std::ffi::CStr;
use std::net::{SocketAddr, ToSocketAddrs};
use std::io::{Error, ErrorKind};
use futures::{Async, Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

struct Writer {
    socket: UdpSocket,
    server_addr: SocketAddr,
    buffer: Option<String>,
}

pub struct Client {
    core: Core,
    writer: Writer,
}

impl Future for Writer {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Self::Error> {
        loop {
            match (self.socket.poll_write(), &mut self.buffer) {
                (Async::Ready(_), some_buffer @ &mut Some(_)) => {
                    {
                        let buffer_bytes = some_buffer.as_ref().unwrap().as_bytes();
                        let amt = try_nb!(self.socket.send_to(buffer_bytes, &self.server_addr));
                        if buffer_bytes.len() != amt {
                            return Err(Error::new(ErrorKind::Other, "bad write"));
                        }
                    }
                    *some_buffer = None;
                },
                _ => return Ok(Async::NotReady),
            }
        }
    }
}

impl Client {
    pub fn new(socket_addr: &str) -> std::io::Result<Self> {
        let local_port: u16 = 26263;
        let server_port: u16 = 26262;
        let core = try!(Core::new());
        let handle = core.handle();
        let local_socket_addr = try!(("127.0.0.1", local_port).to_socket_addrs()).next().unwrap(); // TODO check unwrap()
        let server_socket_addr = try!((socket_addr, server_port).to_socket_addrs()).next().unwrap(); // TODO check unwrap()
        let socket = UdpSocket::bind(&local_socket_addr, &handle).unwrap(); // TODO check unwrap
        let writer = Writer {
            socket: socket,
            server_addr: server_socket_addr,
            buffer: None
        };
        Ok(Client {
            core: core,
            writer: writer
        })
    }

    pub fn send_params(&mut self, x: f64, y: f64) {
        let data = format!("{}\t{}", x, y);
        self.writer.buffer = Some(data);
        self.core.run(&mut self.writer).unwrap();
    }
}

#[no_mangle]
pub extern fn spiro_client_new(addr: *const c_char) -> *mut Client {
    assert!(!addr.is_null());
    let addr = unsafe { CStr::from_ptr(addr).to_string_lossy().into_owned() };
    Box::into_raw(Box::new(Client::new(&addr).unwrap()))
}

#[no_mangle]
pub extern fn spiro_client_send(client: *mut Client, x: f64, y: f64) {
    assert!(!client.is_null());
    let client = unsafe { &mut *client };
    client.send_params(x, y);
}

#[test]
fn connect_to_server() {
    let mut client = match Client::new("127.0.0.1") {
        Ok(client) => client,
        Err(e) => panic!("{:?}", e)
    };
    client.send_params(1.2, 3.4);
}