# Endpoint

`Endpoint` is a Rust crate for parsing, representing, and resolving network and file endpoints in a unified way. It supports a wide range of protocols, including TCP, UDP, HTTP(S), WebSocket, MQTT(S), CoAP(S), Redis, AMQP, FTP, Unix sockets, and local files.

It can parse strings like `"tcp://127.0.0.1:8080"`, `"mqtt://broker.local:1883"`, or `"unix:///tmp/socket.sock"` into a strongly-typed `Endpoint` enum, handle IP and domain hosts, and convert to `SocketAddr` for network connections.

---

## Features

* Parse endpoints from strings into an enum representation.
* Support for IP addresses and domain names.
* Automatically handle default ports for common protocols.
* Convert network endpoints into `SocketAddr`s, resolving domain names via DNS.
* Support local file paths and Unix domain sockets.
* Easy-to-use `Display` formatting for converting back to URI strings.

---

## Supported Protocols

* Network: `tcp`, `udp`, `http`, `https`, `ws`, `wss`, `mqtt`, `mqtts`, `coap`, `coaps`, `redis`, `amqp`, `ftp`
* File/IPC: `unix`, `file`

---

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
endpoint = { git = "https://github.com/loganliug/endpoint.git" } # or use a version from crates.io
```
Parse endpoints:

```rust
use endpoint::{Endpoint, HostAddr};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_ep: Endpoint = "tcp://127.0.0.1:9000".parse()?;
    let mqtt_ep: Endpoint = "mqtt://broker.local:1883".parse()?;
    let unix_ep: Endpoint = "unix:///tmp/socket.sock".parse()?;

    println!("{}", tcp_ep);   // tcp://127.0.0.1:9000
    println!("{}", mqtt_ep);  // mqtt://broker.local:1883
    println!("{}", unix_ep);  // unix:///tmp/socket.sock

    // Convert to SocketAddr
    let addrs = tcp_ep.to_socket_addrs()?;
    for addr in addrs {
        println!("{}", addr);
    }

    Ok(())
}
```

---

## Enum Reference

```rust
pub enum Endpoint {
    Http  { host: HostAddr, port: u16 },
    Https { host: HostAddr, port: u16 },
    Tcp   { host: HostAddr, port: u16 },
    Udp   { host: HostAddr, port: u16 },
    Mqtt  { host: HostAddr, port: u16 },
    Mqtts { host: HostAddr, port: u16 },
    Ws    { host: HostAddr, port: u16 },
    Wss   { host: HostAddr, port: u16 },
    Coap  { host: HostAddr, port: u16 },
    Coaps { host: HostAddr, port: u16 },
    Redis { host: HostAddr, port: u16 },
    Amqp  { host: HostAddr, port: u16 },
    Ftp   { host: HostAddr, port: u16 },
    Unix  (PathBuf),
    File  (PathBuf),
}

pub enum HostAddr {
    Ip(IpAddr),
    Domain(String),
}
```

---

## Error Handling

All parsing errors return a `ParseEndpointError`:

```rust
use endpoint::ParseEndpointError;

match "tcp://:8080".parse::<Endpoint>() {
    Ok(ep) => println!("Parsed: {}", ep),
    Err(ParseEndpointError::InvalidAddress(s)) => println!("Invalid address: {}", s),
    Err(ParseEndpointError::InvalidScheme) => println!("Unsupported scheme"),
}
```

---

## License

MIT / Apache-2.0
