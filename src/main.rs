mod network_manager;
mod testing;

#[cfg(feature = "udp-networking")]
mod udp_network_interface;

#[cfg(feature = "sim-networking")]
mod test_network_interface;

fn main() {
    let connection: network_manager::Connection<'_> = network_manager::connect("192.168.1.2", 8080);
}