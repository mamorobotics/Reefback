#[cfg(feature = "udp-networking")]
use std::{net::UdpSocket, sync::Arc};

pub trait NetworkInterface {
    fn bind_to_address(&mut self, addr: &str);
    fn send_to(&self, msg: &str, addr: &str, port: u16) -> bool;
    fn recv(&self, buf: &mut Vec<u8>) -> (i32, String);
}

#[cfg(feature = "udp-networking")]
#[derive(Clone)]
pub struct UdpNetworkInterface {
    pub socket: Option<Arc<UdpSocket>>
}

#[cfg(feature = "sim-networking")]
#[derive(Clone)]
pub struct SimNetworkInterface {
    pub socket: String
}

#[cfg(feature = "udp-networking")]
impl UdpNetworkInterface {
    pub fn new() -> Self {
        Self { socket: None }
    }
}

#[cfg(feature = "udp-networking")]
impl NetworkInterface for UdpNetworkInterface {
    fn bind_to_address(&mut self, addr: &str) {
        self.socket = Some(Arc::new(UdpSocket::bind(addr).unwrap()));
    }

    fn send_to(&self, msg: &str, addr: &str, port: u16) -> bool {
        let socket = match &self.socket {
            Some(s) => s,
            None => return false, // or panic with a helpful message
        };

        let mut sent = 0;
        let mut remaining = msg.as_bytes();
        let target_addr = format!("{}:{}", addr, port);

        while remaining.len() > 65500 {
            let (chunk, rest) = remaining.split_at(65500);
            sent += socket.send_to(chunk, &target_addr).unwrap();
            remaining = rest;
        }

        if !remaining.is_empty() {
            sent += socket.send_to(remaining, &target_addr).unwrap();
        }

        return sent == msg.as_bytes().len();
    }

    fn recv(&self, buf: &mut Vec<u8>) -> (i32, String) {
        let socket = match &self.socket {
            Some(s) => s,
            None => panic!("Socket not initialized. Call bind_to_address first."),
        };

        let (num_bytes, src_addr) = socket.recv_from(buf).unwrap();
        return (num_bytes.try_into().unwrap(), src_addr.ip().to_string());
    }

}

#[cfg(feature = "sim-networking")]
impl NetworkInterface for SimNetworkInterface {
    fn bind_to_address(&self, addr: &str) {
        self.socket = addr.to_owned();
    }

    fn send_to(&self, msg: &str, _addr: &str, _port: u16) -> bool {
        println!("SENT MESSAGE = {}", msg);
        return true;
    }

    fn recv(&self, buf: &mut Vec<u8>) -> (i32, String) {
        let msg: Vec<u8> = "32!0".as_bytes().to_vec();
        buf.clear();
        buf.extend_from_slice(&msg);

        return (msg.len().try_into().unwrap(), "TEST ADDR".to_owned());
    }
}
