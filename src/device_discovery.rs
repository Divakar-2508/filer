use std::{io, net::UdpSocket};

use crate::models::Device;

const BUFFER_SIZE: usize = 1024;

pub fn get_device_list() -> io::Result<Vec<String>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    socket.connect("192.168.1.255:0")?;
    socket.send(b"DISCOVERY_REQUEST")?;

    let mut buffer = [0u8; BUFFER_SIZE];

    let mut devices: Vec<Device> = Vec::new();

    for _ in 0..10 {
        match socket.recv_from(&mut buffer) {
            Ok((len, addr)) => {
                let name = String::from_utf8_lossy(&buffer[..len]).to_string();
                devices.push(Device { name, addr });
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(vec!["hello".to_owned()])
}

pub fn response_discovery(device_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let mut buffer = [0u8; BUFFER_SIZE];
    let device = Device {
        name: device_name,
        addr: socket.peer_addr()?,
    };
    let device_b = serde_json::to_vec(&device)?;

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((len, addr)) => {
                let msg = String::from_utf8_lossy(&buffer[..len]).to_string();
                if msg == "DISCOVERY_REQUEST" {
                    socket.send_to(&device_b, addr)?;
                }
            }
            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
    }
}
