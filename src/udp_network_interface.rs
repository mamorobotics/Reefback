use std::{net::{SocketAddr, UdpSocket}};

pub struct Socket {
    pub socket: UdpSocket,
}

pub fn bind_to_address(addr: &str) -> Socket {
    return Socket{socket: UdpSocket::bind(addr).unwrap()};
}

pub fn send_to(socket: &Socket, msg: &str, addr: &str, port: u16) -> bool {
    let mut sent = 0;

    let mut msg_data: String = (*msg).to_string();
    
    while msg_data.len() > 65500 {
        let temp = msg_data[..65500].to_string();
        msg_data = msg_data[65500..].to_string();
        sent += socket.socket.send_to(temp.as_bytes(), addr.to_owned() + ":" + &port.to_string()).unwrap();
    } if msg_data.len() != 0 {
        sent += socket.socket.send_to(msg_data.as_bytes(), addr.to_owned() + ":" + &port.to_string()).unwrap();
    }
    return sent == msg.as_bytes().len();
}

pub fn recv(socket: &Socket, buf: &mut Vec<u8>) {
    socket.socket.recv_from(buf).unwrap();
}