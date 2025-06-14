use std::{net::UdpSocket, sync::Arc};

#[derive(Clone)]
pub struct Socket {
    pub socket: Arc<UdpSocket>,
}

pub fn bind_to_address(addr: &str) -> Socket {
    return Socket{socket: Arc::new(UdpSocket::bind(addr).unwrap())};
}

pub fn send_to(socket: &Socket, msg: &str, addr: &str, port: u16) -> bool {
    let mut sent = 0;
    let mut remaining = msg.as_bytes();
    let target_addr = format!("{}:{}", addr, port);
    
    while remaining.len() > 65500 {
        let (chunk, rest) = remaining.split_at(65500);
        sent += socket.socket.send_to(chunk, &target_addr).unwrap();
        remaining = rest;
    }
    
    if !remaining.is_empty() {
        sent += socket.socket.send_to(remaining, &target_addr).unwrap();
    }
    
    return sent == msg.as_bytes().len()
}

pub fn recv<'a>(socket: &'a Socket, buf: &'a mut Vec<u8>) -> (i32, String) {
    let (num_bytes, src_addr) = socket.socket.recv_from(buf).unwrap();
    return (num_bytes.try_into().unwrap(), src_addr.ip().to_string());
}