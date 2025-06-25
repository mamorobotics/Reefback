use std::{
    iter::repeat,
    str,
    sync::{Arc, Mutex},
    thread,
};

use crate::network_interface::NetworkInterface;

#[cfg(feature = "udp-networking")]
static MAX_LEN: usize = 65500;

#[cfg(feature = "sim-networking")]
static MAX_LEN: usize = i32::MAX as usize;

const HOST: bool = cfg!(feature = "host");

static FUNCT_MAP: Mutex<Vec<fn(&str)>> = Mutex::new(Vec::new());
static CONNECTED_ADDRS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub struct Connection<T: NetworkInterface + Send + Sync + Clone + 'static> {
    pub interface: T,
    pub addr: String,
    pub port: u16
}

impl<T: NetworkInterface + Send + Sync + Clone + 'static> Connection<T> {
    pub fn create_persistent_connection(&mut self) -> bool {
        //Socket Creation

        self.interface.bind_to_address(&(self.addr.to_owned() + ":" + &self.port.to_string()));

        //Handshake

        if HOST {
            let mut message_buf: Vec<u8> = vec![0; 32];
            let recv_values = self.interface.recv(&mut message_buf);
            let msg = str::from_utf8(&message_buf).unwrap();

            if msg != "0110" {
                println!("Handshake Failed");
            } else {
                CONNECTED_ADDRS.lock().unwrap().push(recv_values.1);
            }
        } else {
            self.interface.send_to("0110", &self.addr, self.port);
        }

        //Listening

        let interface_stable: Arc<T> = Arc::new(self.interface.clone());

        thread::spawn(move || {
            let mut listening = true;
            while listening {
                let mut init_buf: Vec<u8> = vec![0; 32];
                interface_stable.recv(&mut init_buf);

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
                    interface_stable.recv(&mut temp_buf);
                    total_size += if (size - total_size) > (MAX_LEN as i32) {
                        MAX_LEN as i32
                    } else {
                        size - total_size
                    };
                    data_buf.append(&mut temp_buf);
                }

                let data: Vec<u8> = data_buf;

                for head in (*headers).split("?") {
                    if head == "STOP" {
                        listening = false;
                    }
                    else 
                    {
                        let header_value = head.parse::<usize>().unwrap();
                        FUNCT_MAP.lock().unwrap()[header_value](str::from_utf8(&data).unwrap());
                    }
                }
            }
        });
        
        return true;
    }

    pub fn send_temporary_connection(&mut self, addr: &str, msg: &str, headers: &[&str]) -> bool {
        //Socket Creation

        self.interface.bind_to_address(&(self.addr.to_owned() + ":" + &self.port.to_string()));

        //Handshake

        if HOST {
            let mut message_buf: Vec<u8> = vec![0; 32];
            let recv_values = self.interface.recv(&mut message_buf);
            let msg = str::from_utf8(&message_buf).unwrap();

            if msg != "0110" {
                println!("Handshake Failed");
            } else {
                CONNECTED_ADDRS.lock().unwrap().push(recv_values.1);
            }
        } else {
            self.interface.send_to("0110", &self.addr, self.port);
        }

        if !CONNECTED_ADDRS.lock().unwrap().contains(&(msg.to_owned())) {
            return false;
        }

        //Sending

        let sent = self.send(addr, msg, headers);
        let stopped = self.send(addr, "", &["STOP"]);
        return sent && stopped;
    }

    pub fn send(&self, addr: &str, msg: &str, headers: &[&str]) -> bool {
        if !CONNECTED_ADDRS.lock().unwrap().contains(&(msg.to_owned())) {
            return false;
        }

        if (msg.len().to_string() + "!" + &headers.join("?")).len() > 32 {
            return false;
        }

        let mut pre_msg: String = msg.len().to_string() + "!" + &headers.join("?");
        let pre_length = pre_msg.len();
        pre_msg = pre_msg + &repeat(" ").take(32 - pre_length).collect::<String>();
        let pre_check: bool = self.interface.send_to(&pre_msg, addr, self.port);

        let mut sent = true;
        let mut msg_data: String = (*msg).to_string();
        while msg_data.len() > MAX_LEN {
            let temp = msg_data[..MAX_LEN].to_string();
            msg_data = msg_data[MAX_LEN..].to_string();
            sent &= self.interface.send_to(&temp, addr, self.port)
        }
        if msg_data.len() != 0 {
            sent &= self.interface.send_to(&msg_data, addr, self.port)
        }

        let msg_check: bool = sent;
        return pre_check && msg_check;
    }

    pub fn terminate_connection(&self, addr: &str) -> bool{
        return self.send(addr, "", &["STOP"]);
    }
}

pub fn register_recieve_command(id: usize, command: fn(&str)) {
    FUNCT_MAP.lock().unwrap().insert(id, command);
}
