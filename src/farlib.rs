use std::io::{BufRead, BufReader, Read};

pub struct FarArchive {
    pub version: u8
}

// test for FAR!byAZ
pub fn test(file : Vec<u8>) -> Result<FarArchive, String> {
    let mut reader = BufReader::new(&file[..]);
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic).unwrap();
    if magic != *b"FAR!byAZ" {
        return Err("Not a Far archive".to_string());
    }
    let mut version = [0; 1];
    reader.read_exact(&mut version).unwrap();
    Ok(FarArchive {
        version: version[0]
    })
}