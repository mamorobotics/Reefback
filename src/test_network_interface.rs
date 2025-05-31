use std::{net::{SocketAddr, UdpSocket}};

pub struct Socket {
    pub addr: String,
}

pub fn bind_to_address(addr: &str) -> Socket {
    return Socket{addr: addr.to_string()};
}

pub fn send_to(socket: &Socket, msg: &str, addr: &str, port: u16) -> bool {
    println!("{}", msg);
    return true;
}

pub fn recv(socket: &Socket, buf: &mut Vec<u8>) {
    let msg: Vec<u8> = "!TEST PAYLOAD!".as_bytes().to_vec();
    buf.clear();
    buf.extend_from_slice(&msg);
}