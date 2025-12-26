use std::fmt;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;
use url::Url;

/// Represents a host, either an IP address or a domain name.
#[derive(Debug, Clone, PartialEq)]
pub enum HostAddr {
    Ip(IpAddr),
    Domain(String),
}

/// Represents network or file endpoints.
#[derive(Debug, Clone, PartialEq)]
pub enum Endpoint {
    Http { host: HostAddr, port: u16 },
    Https { host: HostAddr, port: u16 },
    Tcp { host: HostAddr, port: u16 },
    Udp { host: HostAddr, port: u16 },
    Mqtt { host: HostAddr, port: u16 },
    Mqtts { host: HostAddr, port: u16 },
    Ws { host: HostAddr, port: u16 },
    Wss { host: HostAddr, port: u16 },
    Coap { host: HostAddr, port: u16 },
    Coaps { host: HostAddr, port: u16 },
    Redis { host: HostAddr, port: u16 },
    Amqp { host: HostAddr, port: u16 },
    Ftp { host: HostAddr, port: u16 },
    Unix(PathBuf),
    File(PathBuf),
}

/// Errors that can occur when parsing an endpoint.
#[derive(Debug, Error)]
pub enum ParseEndpointError {
    #[error("unsupported scheme")]
    InvalidScheme,
    #[error("invalid address: {0}")]
    InvalidAddress(String),
}

impl FromStr for Endpoint {
    type Err = ParseEndpointError;

    /// Parse a string into an Endpoint enum.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Unix socket
        if let Some(path) = s.strip_prefix("unix://") {
            return Ok(Endpoint::Unix(PathBuf::from(path)));
        }

        // File path
        if let Some(path) = s.strip_prefix("file://") {
            return Ok(Endpoint::File(PathBuf::from(path)));
        }

        // Parse with URL for host/port protocols
        let url = Url::parse(s).map_err(|_| ParseEndpointError::InvalidAddress(s.to_string()))?;
        let host_str = url
            .host_str()
            .ok_or(ParseEndpointError::InvalidAddress(s.to_string()))?;

        let host = if let Ok(ip) = host_str.parse::<IpAddr>() {
            HostAddr::Ip(ip)
        } else {
            HostAddr::Domain(host_str.to_string())
        };

        let port = match url.port() {
            Some(p) => p,
            None => match url.scheme() {
                "http" | "ws" | "mqtt" | "coap" | "redis" | "amqp" | "ftp" => 80,
                "https" | "wss" | "mqtts" | "coaps" => 443,
                _ => return Err(ParseEndpointError::InvalidAddress(s.to_string())),
            },
        };

        match url.scheme() {
            "http" => Ok(Endpoint::Http { host, port }),
            "https" => Ok(Endpoint::Https { host, port }),
            "tcp" => Ok(Endpoint::Tcp { host, port }),
            "udp" => Ok(Endpoint::Udp { host, port }),
            "mqtt" => Ok(Endpoint::Mqtt { host, port }),
            "mqtts" => Ok(Endpoint::Mqtts { host, port }),
            "ws" => Ok(Endpoint::Ws { host, port }),
            "wss" => Ok(Endpoint::Wss { host, port }),
            "coap" => Ok(Endpoint::Coap { host, port }),
            "coaps" => Ok(Endpoint::Coaps { host, port }),
            "redis" => Ok(Endpoint::Redis { host, port }),
            "amqp" => Ok(Endpoint::Amqp { host, port }),
            "ftp" => Ok(Endpoint::Ftp { host, port }),
            _ => Err(ParseEndpointError::InvalidScheme),
        }
    }
}

// Display for Endpoint
impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Endpoint::Http { host, port } => write!(f, "http://{}:{}", host, port),
            Endpoint::Https { host, port } => write!(f, "https://{}:{}", host, port),
            Endpoint::Tcp { host, port } => write!(f, "tcp://{}:{}", host, port),
            Endpoint::Udp { host, port } => write!(f, "udp://{}:{}", host, port),
            Endpoint::Mqtt { host, port } => write!(f, "mqtt://{}:{}", host, port),
            Endpoint::Mqtts { host, port } => write!(f, "mqtts://{}:{}", host, port),
            Endpoint::Ws { host, port } => write!(f, "ws://{}:{}", host, port),
            Endpoint::Wss { host, port } => write!(f, "wss://{}:{}", host, port),
            Endpoint::Coap { host, port } => write!(f, "coap://{}:{}", host, port),
            Endpoint::Coaps { host, port } => write!(f, "coaps://{}:{}", host, port),
            Endpoint::Redis { host, port } => write!(f, "redis://{}:{}", host, port),
            Endpoint::Amqp { host, port } => write!(f, "amqp://{}:{}", host, port),
            Endpoint::Ftp { host, port } => write!(f, "ftp://{}:{}", host, port),
            Endpoint::Unix(path) => write!(f, "unix://{}", path.display()),
            Endpoint::File(path) => write!(f, "file://{}", path.display()),
        }
    }
}

// Display for HostAddr
impl fmt::Display for HostAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HostAddr::Ip(ip) => write!(f, "{}", ip),
            HostAddr::Domain(d) => write!(f, "{}", d),
        }
    }
}

impl Endpoint {
    /// Convert to a list of SocketAddr.
    /// IP addresses return directly, domain names are resolved via DNS.
    pub fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>, ParseEndpointError> {
        match self {
            Endpoint::Http { host, port }
            | Endpoint::Https { host, port }
            | Endpoint::Tcp { host, port }
            | Endpoint::Udp { host, port }
            | Endpoint::Mqtt { host, port }
            | Endpoint::Mqtts { host, port }
            | Endpoint::Ws { host, port }
            | Endpoint::Wss { host, port }
            | Endpoint::Coap { host, port }
            | Endpoint::Coaps { host, port }
            | Endpoint::Redis { host, port }
            | Endpoint::Amqp { host, port }
            | Endpoint::Ftp { host, port } => match host {
                HostAddr::Ip(ip) => Ok(vec![SocketAddr::new(*ip, *port)]),
                HostAddr::Domain(d) => {
                    let addrs: Vec<SocketAddr> = format!("{}:{}", d, port)
                        .to_socket_addrs()
                        .map_err(|_| ParseEndpointError::InvalidAddress(d.clone()))?
                        .collect();
                    Ok(addrs)
                }
            },
            _ => Err(ParseEndpointError::InvalidAddress(
                "No SocketAddr available".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ip_endpoints() {
        let ep: Endpoint = "tcp://127.0.0.1:9000".parse().unwrap();
        assert_eq!(
            ep,
            Endpoint::Tcp {
                host: HostAddr::Ip("127.0.0.1".parse().unwrap()),
                port: 9000
            }
        );
        assert_eq!(ep.to_string(), "tcp://127.0.0.1:9000");
    }

    #[test]
    fn test_parse_domain_endpoints() {
        let ep: Endpoint = "tcp://example.com:8080".parse().unwrap();
        match ep {
            Endpoint::Tcp { host, port } => {
                assert_eq!(port, 8080);
                match host {
                    HostAddr::Domain(d) => assert_eq!(d, "example.com"),
                    _ => panic!("expected domain"),
                }
            }
            _ => panic!("expected TCP endpoint"),
        }
    }

    #[test]
    fn test_parse_unix_and_file() {
        let ep: Endpoint = "unix:///tmp/socket.sock".parse().unwrap();
        match ep {
            Endpoint::Unix(path) => assert_eq!(path, PathBuf::from("/tmp/socket.sock")),
            _ => panic!("expected Unix endpoint"),
        }

        let ep: Endpoint = "file:///home/user/data.txt".parse().unwrap();
        match ep {
            Endpoint::File(path) => assert_eq!(path, PathBuf::from("/home/user/data.txt")),
            _ => panic!("expected File endpoint"),
        }
    }

    #[test]
    fn test_parse_mqtt_ws_endpoints() {
        let ep: Endpoint = "mqtt://broker.local:1883".parse().unwrap();
        match ep {
            Endpoint::Mqtt { host, port } => {
                assert_eq!(port, 1883);
                match host {
                    HostAddr::Domain(d) => assert_eq!(d, "broker.local"),
                    _ => panic!("expected domain"),
                }
            }
            _ => panic!("expected MQTT endpoint"),
        }

        let ep: Endpoint = "wss://example.com:443".parse().unwrap();
        match ep {
            Endpoint::Wss { host, port } => {
                assert_eq!(port, 443);
                match host {
                    HostAddr::Domain(d) => assert_eq!(d, "example.com"),
                    _ => panic!("expected domain"),
                }
            }
            _ => panic!("expected WSS endpoint"),
        }
    }

    #[test]
    fn test_to_socket_addrs_ip() {
        let ep: Endpoint = "tcp://127.0.0.1:9000".parse().unwrap();
        let addrs = ep.to_socket_addrs().unwrap();
        assert_eq!(addrs[0], "127.0.0.1:9000".parse::<SocketAddr>().unwrap());
    }
}
