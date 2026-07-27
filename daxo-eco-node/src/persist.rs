use std::fs::File;
use std::io::{Read, Write};

pub fn load_seq(path: &str) -> Option<u64> {
    let mut file = File::open(path).ok()?;
    let mut buf = [0u8; 8];
    file.read_exact(&mut buf).ok()?;
    Some(u64::from_le_bytes(buf))
}

pub fn save_seq(path: &str, seq: u64) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&seq.to_le_bytes())?;
    Ok(())
}