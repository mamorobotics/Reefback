mod network_interfaces;
mod network_manager;

fn main() {
    //Create a connection
    let connection = network_manager::connect("192.168.1.2", 8080);

    //Send data
    network_manager::send(&connection, "192.168.1.1", "Hello", &[]);

    //Register a function to handle recieved data
    network_manager::register_recieve_command(0, |data: &str| println!("{}", data));
}
