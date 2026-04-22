//! Need to rewrite internals of this
//! this is bad code, I know.

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
struct RawConnConfig {
    ip: Option<String>,
    port: Option<u16>,
}

#[derive(Debug, Deserialize, Default)]
struct RawServerConfig {
    https: Option<RawConnConfig>,
    websockets: Option<RawConnConfig>,
}

#[derive(Debug, Deserialize, Default)]
struct RawConfig {
    servers: Option<RawServerConfig>,
}

#[derive(Debug)]
pub struct ConnConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub https: ConnConfig,
    pub websockets: ConnConfig,
}

#[derive(Debug)]
pub struct Config {
    pub servers: ServerConfig,
}

impl Default for ConnConfig {
    fn default() -> Self {
        Self {
            ip: "0.0.0.0".to_string(),
            port: 5959,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            https: ConnConfig::default(),
            websockets: ConnConfig {
                port: 9001,
                ..ConnConfig::default()
            },
        }
    }
}

impl From<RawConnConfig> for ConnConfig {
    fn from(raw: RawConnConfig) -> Self {
        let default = ConnConfig::default();
        Self {
            ip: raw.ip.unwrap_or(default.ip),
            port: raw.port.unwrap_or(default.port),
        }
    }
}

impl From<RawServerConfig> for ServerConfig {
    fn from(raw: RawServerConfig) -> Self {
        let default = ServerConfig::default();
        Self {
            https: raw.https.map(Into::into).unwrap_or(default.https),
            websockets: raw.websockets.map(Into::into).unwrap_or(default.websockets),
        }
    }
}

impl From<RawConfig> for Config {
    fn from(raw: RawConfig) -> Self {
        Self {
            servers: raw.servers.map(Into::into).unwrap_or_default(),
        }
    }
}

pub fn load_config() -> Config {
    let raw: RawConfig = std::fs::read_to_string("config.toml")
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default();

    raw.into()
}
