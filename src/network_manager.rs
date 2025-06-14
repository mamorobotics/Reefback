use std::{
    iter::repeat,
    str,
    sync::{Arc, Mutex},
    thread,
};

//Imports based on network type selection
#[cfg(feature = "udp-networking")]
use crate::network_interfaces::udp_network_interface::*;

#[cfg(feature = "udp-networking")]
static MAX_LEN: usize = 65500;

#[cfg(feature = "sim-networking")]
use crate::network_interfaces::test_network_interface::*;

#[cfg(feature = "sim-networking")]
static MAX_LEN: usize = i32::MAX as usize;

const HOST: bool = cfg!(feature = "host");

static FUNCT_MAP: Mutex<Vec<fn(&str)>> = Mutex::new(Vec::new());
static CONNECTED_ADDRS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub struct Connection<'a> {
    socket: Arc<Socket>,
    addr: &'a str,
    port: u16,
}

pub fn connect(addr: &str, port: u16) -> Connection {
    //Socket Creation
    let socket: Socket = bind_to_address(&(addr.to_owned() + ":" + &port.to_string()));

    let arc_socket = Arc::new(socket.clone());

    //Handshake
    if HOST {
        let mut message_buf: Vec<u8> = vec![0; 32];
        let recv_values = recv(&(socket.clone()), &mut message_buf);
        let msg = str::from_utf8(&message_buf).unwrap();

        if msg != "0110" {
            println!("Handshake Failed");
        } else {
            CONNECTED_ADDRS.lock().unwrap().push(recv_values.1);
        }
    } else {
        send_to(&(socket.clone()), "0110", addr, port);
    }

    let socket_stable = Arc::clone(&arc_socket);

    thread::spawn(move || {
        loop {
            let mut init_buf: Vec<u8> = vec![0; 32];
            recv(&socket_stable, &mut init_buf);

            let msg = str::from_utf8(&init_buf).unwrap();

            let size: &i32 = &msg[..msg.rfind("!").unwrap()].parse::<i32>().unwrap();
            let headers = &msg[(msg.rfind("!").unwrap() + 1)..];

            let mut data_buf: Vec<u8> = Vec::new();

            let mut total_size: i32 = 0;
            while total_size < size - 1 {
                let mut temp_buf: Vec<u8> = Vec::new();
                temp_buf.resize(
                    if (size - total_size) > (MAX_LEN as i32) {
                        MAX_LEN
                    } else {
                        (size - total_size) as usize
                    },
                    0,
                );
                recv(&socket_stable, &mut temp_buf);
                total_size += if (size - total_size) > (MAX_LEN as i32) {
                    MAX_LEN as i32
                } else {
                    size - total_size
                };
                data_buf.append(&mut temp_buf);
            }

            let data: Vec<u8> = data_buf;

            for head in (*headers).split("?") {
                let header_value = head.parse::<usize>().unwrap();
                FUNCT_MAP.lock().unwrap()[header_value](str::from_utf8(&data).unwrap());
            }
        }
    });

    return Connection {
        socket: arc_socket,
        addr: addr,
        port: port,
    };
}

pub fn send(connection: &Connection, addr: &str, msg: &str, headers: &[&str]) -> bool {
    if !CONNECTED_ADDRS.lock().unwrap().contains(&(msg.to_owned())) {
        return false;
    }

    let mut pre_msg: String = msg.len().to_string() + "!" + &headers.join("?");
    let pre_length = pre_msg.len();
    pre_msg = pre_msg + &repeat(" ").take(32 - pre_length).collect::<String>();
    let pre_check: bool = send_to(&connection.socket, &pre_msg, addr, connection.port);

    let mut sent = true;
    let mut msg_data: String = (*msg).to_string();
    while msg_data.len() > MAX_LEN {
        let temp = msg_data[..MAX_LEN].to_string();
        msg_data = msg_data[MAX_LEN..].to_string();
        sent &= send_to(&connection.socket, &temp, addr, connection.port)
    }
    if msg_data.len() != 0 {
        sent &= send_to(&connection.socket, &msg_data, addr, connection.port)
    }

    let msg_check: bool = sent;
    return pre_check && msg_check;
}

pub fn register_recieve_command(id: usize, command: fn(&str)) {
    FUNCT_MAP.lock().unwrap().insert(id, command);
}
