use std::net::SocketAddr;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
	pub addr: SocketAddr
}
