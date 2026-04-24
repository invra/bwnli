use axum::{Router, routing::get};
use std::io::ErrorKind;
use tracing::{error, info};

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
    let server = tokio::net::TcpListener::bind(&target).await;
    let app = Router::new().route("/", get(root));

    async fn root() -> &'static str {
        "Hello, World!"
    }

    match server {
        Ok(server) => {
            info!("Binded to address {}.", &target);
            axum::serve(server, app).await.unwrap();
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
            // Print out enum's `message` field.
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
