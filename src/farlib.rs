use std::io::{BufRead, BufReader, Read};

pub struct FarFileInfo {
    pub name: String,
    pub size: u32,
    offset: u32,
}

pub struct FarFile {
    pub name: String,
    pub size: u32,
    pub data: Vec<u8>,
}

pub struct FarArchive {
    pub version: u8,
    pub file_count: u32,
    pub file_list: Vec<FarFileInfo>,
    pub file_data: Vec<FarFile>,
}

impl FarFile {
    pub fn new_from_archive(name : String, size : u32, offset : u32, original_file : &Vec<u8>) -> FarFile {
        let mut reader = BufReader::new(&original_file[offset as usize..(offset + size) as usize]);
        let mut data = Vec::new();
        reader.read_to_end(&mut data);
        FarFile {
            name,
            size,
            data,
        }
    }
}

impl FarArchive {
    pub fn load_file_data(self, original_file : &Vec<u8>) -> FarArchive {
        let mut new_file_data = Vec::new();
        for i in 0..self.file_list.len() {
            new_file_data.push(FarFile::new_from_archive(
                self.file_list[i].name.clone(),
                self.file_list[i].size,
                self.file_list[i].offset,
                original_file,
            ));
        }
        FarArchive {
            version: self.version,
            file_count: self.file_count,
            file_list: self.file_list,
            file_data: new_file_data,
        }
    }
}

// test for FAR!byAZ
pub fn test(file : &Vec<u8>) -> Result<FarArchive, String> {
    let mut reader = BufReader::new(&file[..]);
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic).unwrap();
    if magic != *b"FAR!byAZ" {
        return Err("Not a Far archive".to_string());
    }
    let mut version = [0; 1];
    reader.read_exact(&mut version).unwrap();
    let version = version[0];
    // get list of files
    let files = list_files(file).expect("Failed to list files");
    Ok(FarArchive {
        version,
        file_count: files.len() as u32,
        file_list: files,
        file_data: vec![],
    })
}

pub fn list_files(file : &Vec<u8>) -> Result<Vec<FarFileInfo>, String> {
    // manifest offset is at 12 bytes (u32)
    let mut reader = BufReader::new(&file[12..]);
    let mut offset = [0u8; 4];
    reader.read_exact(&mut offset).unwrap();
    let offset = u32::from_le_bytes(offset);
    // move to manifest
    reader = BufReader::new(&file[offset as usize..]);
    // read u32 for number of files
    let mut num_files = [0u8; 4];
    reader.read_exact(&mut num_files).unwrap();
    let num_files = u32::from_le_bytes(num_files);
    // for each file, read u32 for size, u32 for size again (stored twice for some reason), u32 for offset, u32 for name length, name
    let mut files = Vec::new();
    for i in 0..num_files {
        let mut size = [0u8; 4];
        reader.read_exact(&mut size).expect(format!("Failed to read size for file {}", i).as_str());
        let size = u32::from_le_bytes(size);
        let mut size2 = [0u8; 4];
        reader.read_exact(&mut size2).expect(format!("Failed to read size for file {}", i).as_str());
        let size2 = u32::from_le_bytes(size2);
        let mut offset = [0u8; 4];
        reader.read_exact(&mut offset).expect(format!("Failed to read offset for file {}", i).as_str());
        let offset = u32::from_le_bytes(offset);
        let mut name_len = [0u8; 4];
        reader.read_exact(&mut name_len).expect(format!("Failed to read name length for file {}", i).as_str());
        let name_len = u32::from_le_bytes(name_len);
        let mut name = vec![0u8; name_len as usize];
        reader.read_exact(&mut name).expect(format!("Failed to read name for file {}", i).as_str());
        files.push(FarFileInfo {
            name: String::from_utf8(name).unwrap(),
            size,
            offset,
        });
    }
    Ok(files)
}