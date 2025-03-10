use std::{io, path::Path};

pub fn to_n_bytes(content: &str, n: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; n];
    let content_slice = content.as_bytes();
    let len = content_slice.len().min(n);

    buffer[..len].copy_from_slice(&content_slice[..len]);

    buffer
}

const KB: usize = 1024;
const MB: usize = 1024 * KB;
const GB: usize = 1024 * MB;

pub fn determine_chunk_size<P: AsRef<Path>>(path: P) -> io::Result<usize> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(io::ErrorKind::NotFound.into());
    }

    let file_size = path.metadata()?.len() as usize;

    if file_size > 10 * GB {
        Ok(32 * MB)
    } else if file_size > GB {
        Ok(16 * MB)
    } else if file_size > 100 * MB {
        Ok(4 * MB)
    } else if file_size > 10 * MB {
        Ok(MB)
    } else {
        Ok(64 * KB)
    }
}
