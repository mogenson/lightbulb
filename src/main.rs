use gio::prelude::*;
use gtk::prelude::*;
use serde::Serialize;
use std::env::args;
use std::net::UdpSocket;

#[derive(Serialize)]
struct Params {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Serialize)]
#[allow(non_camel_case_types)]
enum Method {
    setPilot,
}

#[derive(Serialize)]
struct Message {
    method: Method,
    params: Params,
}

fn build_ui(application: &gtk::Application) {
    let glade = include_str!("lightbulb.glade");
    let builder = gtk::Builder::new_from_string(glade);
    let window: gtk::ApplicationWindow = builder.get_object("window").expect("can't open window");
    window.set_application(Some(application));

    let button: gtk::Button = builder.get_object("button").expect("can't get button");
    let color: gtk::ColorChooser = builder
        .get_object("color")
        .expect("can't get color chooser");
    let hostname: gtk::Entry = builder
        .get_object("hostname")
        .expect("can't get text entry");

    let socket = UdpSocket::bind("0.0.0.0:0").expect("can't open socket");

    button.connect_clicked(move |_| {
        let rgba: gdk::RGBA = color.get_rgba();
        let text = hostname.get_text().expect("can't get hostname");

        let message = Message {
            method: Method::setPilot,
            params: Params {
                r: (rgba.red * 255.0) as u8,
                g: (rgba.green * 255.0) as u8,
                b: (rgba.blue * 255.0) as u8,
            },
        };

        let json_str = serde_json::to_string(&message).expect("can't convert to json");
        let json_vec = serde_json::to_vec(&message).expect("can't convert to json");

        println!("sending: {} to {}:38899", json_str, text);
        socket
            .send_to(&json_vec, format!("{}:38899", text))
            .expect("can't send udp");
    });

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.mogenson.lightbulb"), Default::default())
            .expect("can't start gtk");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
