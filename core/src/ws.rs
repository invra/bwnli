use futures_util::{SinkExt, StreamExt};
use std::io::ErrorKind;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response}; // via re-export
use tracing::{error, info};

#[derive(Clone, Debug)]
pub struct Addr {
    ip: String,
    port: u16,
}

impl From<config::ConnConfig> for Addr {
    fn from(value: config::ConnConfig) -> Self {
        Self {
            ip: value.ip,
            port: value.port,
        }
    }
}

pub async fn start(address: Addr) -> Result<(), ()> {
    let target = format!("{}:{}", address.ip, address.port);

    match TcpListener::bind(&target).await {
        Ok(server) => {
            info!("Binded to address {}.", &target);
            loop {
                match server.accept().await {
                    Ok((stream, _)) => {
                        tokio::spawn(async move {
                            let callback = |req: &Request, mut response: Response| {
                                info!(
                                    title = "Received a new ws handshake",
                                    description =
                                        format!("path: {}\nheaders:\n{}", req.uri().path(), {
                                            let mut acc = String::new();
                                            for (header, _value) in req.headers() {
                                                acc += &format!("- {header}\n");
                                            }
                                            acc
                                        })
                                );
                                let headers = response.headers_mut();
                                headers.append("MyCustomHeader", ":)".parse().unwrap());
                                headers.append(
                                    "SOME_TUNGSTENITE_HEADER",
                                    "header_value".parse().unwrap(),
                                );
                                Ok(response)
                            };

                            match accept_hdr_async(stream, callback).await {
                                Ok(mut ws) => {
                                    while let Some(msg) = ws.next().await {
                                        match msg {
                                            Ok(msg) if msg.is_binary() || msg.is_text() => {
                                                if ws.send(msg).await.is_err() {
                                                    break;
                                                }
                                            }
                                            _ => break,
                                        }
                                    }
                                }
                                Err(err) => error!("WS handshake failed: {err}"),
                            }
                        });
                    }
                    Err(err) => error!("Failed to accept connection: {err}"),
                }
            }
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
            error!("{}", err.to_string());
            Ok(())
        }
        e => {
            error!("Unhandled exception: {e:#?}");
            Ok(())
        }
    }
}
