#[derive(Clone)]
pub struct Socket {
    pub addr: String,
}

pub fn bind_to_address(addr: &str) -> Socket {
    return Socket{addr: addr.to_owned()};
}

pub fn send_to(_socket: &Socket, msg: &str, _addr: &str, _port: u16) -> bool {
    println!("SENT MESSAGE = {}", msg);
    return true;
}

pub fn recv<'a>(_socket: &'a Socket, buf: &'a mut Vec<u8>) -> (i32, String) {
    let msg: Vec<u8> = "32!0".as_bytes().to_vec();
    buf.clear();
    buf.extend_from_slice(&msg);

    return (msg.len().try_into().unwrap(), "TEST ADDR".to_owned());
}