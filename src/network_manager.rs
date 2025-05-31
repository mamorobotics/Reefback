use std::{str, sync::*, thread};

//Imports based on network type selection
#[cfg(feature = "udp-networking")]
use crate::udp_network_interface::*;

#[cfg(feature = "sim-networking")]
use crate::test_network_interface::*;


const HOST: bool = cfg!(feature = "host");

#[cfg(test)]
pub struct Connection<'a> {
    pub socket: Arc<Socket>,
    pub data: Arc<Vec<u8>>,
    pub addr: &'a str,
    pub port: u16,
}

#[cfg(not(test))]
pub struct Connection<'a> {
    socket: Arc<Socket>,
    data: Arc<Vec<u8>>,
    addr: &'a str,
    port: u16,
}

pub fn connect(addr: &str, port: u16) -> Connection<>{
    //Socket Creation
    let socket: Socket = bind_to_address(&(addr.to_owned() + ":" + &port.to_string()));

    //Handshake
    if HOST {
    
    }
    else
    {
        send_to(&socket, "0110", addr, port);
    }    

    //Listening Thread
    let socket_stable: Arc<Socket> = Arc::new(socket);
    let mut data_stable: Arc<Vec<u8>> = Arc::new(Vec::new());

    thread::scope(|s| {
        s.spawn(||{
            loop
            {
                let mut init_buf: Vec<u8> = Vec::new();
                init_buf.resize(32, 0);
                recv(&socket_stable, &mut init_buf);

                let msg = str::from_utf8(&init_buf).unwrap();

                let size = &msg[..msg.rfind("!").unwrap()].parse::<i32>().unwrap(); 
                let headers = &msg[msg.rfind("!").unwrap()..];

                let mut data_buf: Vec<u8> = Vec::new();

                let mut total_size = 0;
                while total_size < size-1
                {
                    let mut temp_buf: Vec<u8> = Vec::new();
                    temp_buf.resize(if (size - total_size) > 65500{65500} else {(size-total_size) as usize}, 0);
                    recv(&socket_stable, &mut temp_buf);
                    total_size += if (size - total_size) > 65500{65500} else {size-total_size};
                    data_buf.append(&mut temp_buf);
                }

                data_stable = data_buf.into();

                for head in (*headers).split("?"){
                    
                }
            }
        });
    });

    return Connection {socket: socket_stable, data: data_stable, addr: addr, port: port};
}

pub fn send(connection: &Connection, addr: &str, msg: &str) -> bool {
    return send_to(&connection.socket, msg, addr, connection.port);
}

pub fn recieve(connection: &Connection) -> String{
    return String::from_utf8((connection.data).clone().to_vec()).unwrap();
}