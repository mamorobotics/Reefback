use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{network_interface::*, network_manager::*};
use imgui::*;

mod network_interface;
mod network_manager;
mod ui_manager;

fn main() {
    //Initialize the UI
    let args: Mutex<HashMap<String, Mutex<String>>> = Mutex::new(HashMap::new());
    args.lock().unwrap().insert("text".to_owned(), Mutex::new("Wassup?".to_owned()));
    let args_shared = Arc::new(args);

    ui_manager::create_ui(|ui: &mut Ui, args| {
        ui.window("Hello").size([300.0, 100.0], imgui::Condition::FirstUseEver).build(
            || {
                ui.text(&*args.lock().unwrap().get("text").unwrap().lock().unwrap());
            }
        );
    }, args_shared);

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
