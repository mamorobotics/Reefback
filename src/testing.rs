#[cfg(test)]
mod tests {
    use crate::network_manager;

    #[cfg(feature = "udp-networking")]
    use crate::udp_network_interface;

    #[cfg(feature = "sim-networking")]
    use crate::test_network_interface;

    #[test]
    fn checkNetworkAddrCreation() {
        let connection: network_manager::Connection<'_> = network_manager::connect("192.168.1.2", 8080);
        assert!(connection.addr == "192.168.1.2");
    }

    #[test]
    fn checkNetworkPortCreation() {
        let connection: network_manager::Connection<'_> = network_manager::connect("192.168.1.2", 8080);
        assert!(connection.port == 8080);
    }

    #[test]
    fn checkNetworkSend() {
        let connection: network_manager::Connection<'_> = network_manager::connect("192.168.1.2", 8080);
        network_manager::send(&connection, "192.168.1.2:8080", "TEST");
    }

    #[test]
    fn checkNetworkRecieve() {
        let connection: network_manager::Connection<'_> = network_manager::connect("192.168.1.2", 8080);
        assert!(network_manager::recieve(&connection) == "!TEST PAYLOAD!");
    }

}