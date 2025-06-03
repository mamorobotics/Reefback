pub struct Socket {
    pub addr: String,
}

pub fn bind_to_address(addr: &str) -> Socket {
    return Socket{addr: addr.to_string()};
}

pub fn send_to(_socket: &Socket, msg: &str, _addr: &str, _port: u16) -> bool {
    println!("SENT MESSAGE = {}", msg);
    return true;
}

pub fn recv(_socket: &Socket, buf: &mut Vec<u8>) {
    let msg: Vec<u8> = "32!".as_bytes().to_vec();
    buf.clear();
    buf.extend_from_slice(&msg);
}