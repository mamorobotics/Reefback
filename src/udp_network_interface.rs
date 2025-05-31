use std::{net::{SocketAddr, UdpSocket}};

pub struct Socket {
    pub socket: UdpSocket,
}

pub fn bind_to_address(addr: &str) -> Socket {
    return Socket{socket: UdpSocket::bind(addr).unwrap()};
}

pub fn send_to(socket: &Socket, msg: &str, addr: &str, port: u16) -> bool {

    let send: usize = socket.socket.send_to(msg.as_bytes(), addr.to_owned() + ":" + &port.to_string()).unwrap();
    return send == msg.as_bytes().len();
}

pub fn recv(socket: &Socket, buf: &mut Vec<u8>) {
    socket.socket.recv_from(buf).unwrap();
}