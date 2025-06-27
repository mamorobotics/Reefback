use crate::{network_interface::*, network_manager::*};
use imgui::*;

mod network_interface;
mod network_manager;
mod ui_manager;

fn main() {
    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    ui_manager::simple_init(file!(), move |_, ui| {
        ui.window("Hello world")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(|| {
                ui.text_wrapped("Hello world!");
                ui.text_wrapped("こんにちは世界！");
                if ui.button(choices[value]) {
                    value += 1;
                    value %= 2;
                }

                ui.button("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });

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
