use crate::util;
use rand::{distributions::Alphanumeric, Rng};
use std::{
    fs,
    io::{self, Read, Write},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Header {
    file_size: usize,
    chunk_size: usize,
    file_type: String,
    file_name: String,
}

impl Header {
    const HEADER_SIZE: usize = 280;

    pub fn new(file_name: &str, file_type: &str, file_size: usize, chunk_size: usize) -> Self {
        Header {
            file_name: file_name.to_string(),
            file_type: file_type.to_string(),
            file_size,
            chunk_size,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(io::ErrorKind::NotFound.into());
        }

        let meta_data = fs::metadata(path)?;
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| {
                rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(6)
                    .map(char::from)
                    .collect()
            });
        let file_type = path.extension().and_then(|s| s.to_str()).unwrap_or("misc");
        let chunk_size = util::determine_chunk_size(path)?;

        Ok(Header::new(
            &file_name,
            file_type,
            meta_data.len() as usize,
            chunk_size,
        ))
    }

    pub fn to_bytes(&self) -> [u8; Self::HEADER_SIZE] {
        let mut buffer = [0u8; Self::HEADER_SIZE];
        buffer[..8].copy_from_slice(&self.file_size.to_le_bytes());
        buffer[8..16].copy_from_slice(&self.chunk_size.to_le_bytes());
        buffer[16..272].copy_from_slice(&util::to_n_bytes(&self.file_name, 256));
        buffer[272..].copy_from_slice(&util::to_n_bytes(&self.file_type, 8));
        buffer
    }

    pub fn from_bytes(payload: &[u8; 280]) -> Self {
        let file_size = usize::from_le_bytes(payload[0..8].try_into().unwrap());
        let chunk_size = usize::from_le_bytes(payload[8..16].try_into().unwrap());
        let file_name = String::from_utf8_lossy(&payload[16..272]);
        let file_type = String::from_utf8_lossy(&payload[272..]);

        Header::new(
            file_name.trim_end_matches('\0'),
            file_type.trim_end_matches('\0'),
            file_size,
            chunk_size,
        )
    }

    pub fn set_chunk_size(&mut self, chunk_size: usize) {
        self.chunk_size = chunk_size;
    }

    pub fn file_size(&self) -> usize {
        self.file_size
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_type(&self) -> &str {
        &self.file_type
    }
}

pub fn write_header<W: Write>(dest: &mut W, header: &Header) -> io::Result<()> {
    dest.write_all(&header.to_bytes())?;
    Ok(())
}

pub fn read_header<R: Read>(source: &mut R) -> io::Result<Header> {
    let mut buffer = [0u8; Header::HEADER_SIZE];
    source.read_exact(&mut buffer)?;

    Ok(Header::from_bytes(&buffer))
}
