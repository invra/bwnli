use std::{io::ErrorKind, net::TcpListener, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

#[derive(Clone, Debug)]
pub struct Addr {
    ip: String,
    port: u16,
}

impl From<config::ConnConfig> for Addr {
    fn from(value: config::ConnConfig) -> Self {
        let ip = value.ip;
        let port = value.port;
        Self { ip, port }
    }
}

pub async fn start(address: Addr) -> Result<(), ()> {
    let target = format!("{}:{}", address.ip, address.port);
    let server = TcpListener::bind(&target);
    match server {
        // Mask over server current server variable
        Ok(server) => {
            println!("Binded to {}", &target);
            for stream in server.incoming() {
                spawn(move || {
                    let callback = |req: &Request, mut response: Response| {
                        println!("Received a new ws handshake");
                        println!("The request's path is: {}", req.uri().path());
                        println!("The request's headers are:");
                        for (header, _value) in req.headers() {
                            println!("* {header}");
                        }

                        let headers = response.headers_mut();
                        headers.append("MyCustomHeader", ":)".parse().unwrap());
                        headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                        Ok(response)
                    };
                    let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();
                    loop {
                        let msg = websocket.read().unwrap();
                        if msg.is_binary() || msg.is_text() {
                            websocket.send(msg).unwrap();
                        }
                    }
                });
            }
            Ok(())
        }
        Err(err) if err.kind() == ErrorKind::AddrInUse => {
            eprintln!("Binding is already in use.");
            Ok(())
        }
        Err(err) if err.kind() == ErrorKind::AddrNotAvailable => {
            eprintln!("Binding is not available.");
            Ok(())
        }
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            eprintln!("Binding is not allowed.");
            eprintln!("You can try with doas/sudo.");
            Ok(())
        }
        Err(err) if err.kind() == ErrorKind::InvalidInput => {
            // Print out enum's `message` feild.
            // e.g a port is invalid, and will capture
            // "invalid port value"
            eprintln!("{}", err.to_string());
            Ok(())
        }
        e => {
            eprintln!("Unhandled exception: {e:#?}");
            Ok(())
        }
    }
}
