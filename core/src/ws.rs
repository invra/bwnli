use std::{io::ErrorKind, net::TcpListener, thread::spawn};
use tracing::{error, info};

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
            info!("Binded to address {}.", &target);
            for stream in server.incoming() {
                spawn(move || {
                    let callback = |req: &Request, mut response: Response| {
                        info!(
                            title = "Received a new ws handshake",
                            description = format!("path: {}\nheaders:\n{}", req.uri().path(), {
                                let mut acc = String::new();
                                for (header, _value) in req.headers() {
                                    acc += &format!("- {header}\n");
                                }
                                acc
                            })
                        );

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
            error!(
                title = format!("tcp://{}:{} is already in use.", address.ip, address.port),
                description = "Try killing already used binds or\nFind a non-used port"
            );
            Ok(())
        }
        Err(err) if err.kind() == ErrorKind::AddrNotAvailable => {
            error!("tcp://{}:{} is not available.", address.ip, address.port);
            Ok(())
        }
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            error!(
                title = "Permission for Binding is denied.",
                description = "Try running this program with\nelevated privileges."
            );
            Ok(())
        }
        Err(err) if err.kind() == ErrorKind::InvalidInput => {
            // Print out enum's `message` feild.
            // e.g a port is invalid, and will capture
            // "invalid port value"
            error!("{}", err.to_string());
            Ok(())
        }
        e => {
            error!("Unhandled exception: {e:#?}");
            Ok(())
        }
    }
}
