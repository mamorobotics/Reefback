use crate::{network_interface::*, network_manager::*, ui_manager::UiArguments};
use imgui::*;

mod network_interface;
mod network_manager;
mod ui_manager;

fn main() {
    //Initialize the UI
    let args: UiArguments = UiArguments::argument_builder(&[
        ("text","Wassup?"),
        ("Hello", "World")
    ]);

    ui_manager::create_ui(|ui: &mut Ui, args| {
        ui.window("Hello").size([300.0, 100.0], imgui::Condition::FirstUseEver).build(
            || {
                ui.text(args.get_argument_value("text"));
            }
        );
    }, args);

    //Create a connection type
    let mut connection: Connection<SimNetworkInterface> = Connection {
        interface: SimNetworkInterface::new(),
        addr: "192.168.1.2".to_owned(),
        port: 8080,
    };

    //Do stuff with the connection
    connection.create_persistent_connection();
    connection.send("192.168.1.1", "Hello", &[]);

    //Register a function to handle recieved data
    network_manager::register_recieve_command(0, |data: &str| println!("{}", data));
}
