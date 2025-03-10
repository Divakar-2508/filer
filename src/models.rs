use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub addr: SocketAddr,
}
