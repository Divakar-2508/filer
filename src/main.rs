// mod device_discovery;
mod header;
mod models;
mod util;

use header::Header;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    str::FromStr,
};

pub fn transfer_data<W: Write, R: Read>(
    source: &mut R,
    dest: &mut W,
    header: Header,
) -> io::Result<()> {
    let mut buffer = vec![0u8; header.chunk_size()];
    let mut bytes_read = 0;

    while bytes_read < header.file_size() {
        let bytes_to_read = std::cmp::min(header.chunk_size(), header.file_size() - bytes_read);
        source.read_exact(&mut buffer[..bytes_to_read])?;
        dest.write_all(&buffer[..bytes_to_read])?;
        bytes_read += header.chunk_size();
    }

    Ok(())
}

fn send() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    print!("File path: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;

    let file_path = PathBuf::from(&buffer.trim());
    let header = header::Header::from_path(&file_path)?;
    let mut reader = File::open(file_path)?;

    print!("recv address: ");
    io::stdout().flush()?;
    buffer.clear();
    io::stdin().read_line(&mut buffer)?;

    let mut socket = TcpStream::connect(buffer.trim())?;
    header::write_header(&mut socket, &header)?;
    transfer_data(&mut reader, &mut socket, header)?;

    Ok(())
}

fn recv() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    print!("Enter sender addr: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;

    let listener = TcpListener::bind(SocketAddr::from_str(buffer.trim())?)?;
    let (mut socket, addr) = listener.accept()?;
    println!("recving from {}", addr);

    let header = header::read_header(&mut socket)?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(header.file_name())?;

    transfer_data(&mut socket, &mut file, header)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first();

    if mode.is_none() {
        println!("mention mode <send, recv>");
        return Ok(());
    }

    let mode = mode.unwrap().trim().to_lowercase();
    if mode == "send" {
        send()?;
    } else {
        recv()?;
    }

    Ok(())
}
