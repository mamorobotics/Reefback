use crate::{network_interface::*, network_manager::*};

mod network_interface;
mod network_manager;

fn main() {
    //Create a connection type
    let mut connection: Connection<UdpNetworkInterface> = Connection { 
        interface: UdpNetworkInterface::new(),
        addr: "192.168.1.2".to_owned(),
        port: 8080
    };

    //Do stuff with the connection
    connection.create_persistent_connection();
    connection.send("192.168.1.1", "Hello", &[]);

    //Register a function to handle recieved data
    network_manager::register_recieve_command(0, |data: &str| println!("{}", data));
}
