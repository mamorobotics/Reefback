mod network_manager;

#[cfg(feature = "udp-networking")]
mod default_network_interface;

fn main() {
    let connection: network_manager::Connection<'_> = network_manager::connect("192.168.1.2", 8080);
    network_manager::send(&connection, "192.168.1.1");
}