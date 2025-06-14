use std::{net::UdpSocket};

#[derive(Clone)]
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

pub fn recv<'a>(socket: &'a Socket, buf: &'a mut Vec<u8>) -> (i32, String) {
    let (num_bytes, src_addr) = socket.socket.recv_from(buf).unwrap();
    return (num_bytes.try_into().unwrap(), src_addr.ip().to_string());
}