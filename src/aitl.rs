use std::fs;

use anyhow::{Result, bail};
use serde::{Serialize, Deserialize};

pub fn open_AITL_file(path: &str) -> Result<AITLFile> {

    let data = fs::read(path).unwrap();
    
    let mut byte_buffer = Vec::new();

    let mut data = data.into_iter();

    loop {
        match data.next() {
            Some(item) => {
                if item == b'\0' { break; }
                byte_buffer.push(item);
            }
            None => { bail!("Early EOF"); }
        }
    }
    let header_size: usize = String::from_utf8(byte_buffer).unwrap().parse().unwrap();

    let mut header = Vec::new();
    for _ in 0..header_size {
        match data.next() {
            Some(item) => {
                header.push(item);
            }
            None => { bail!("Early EOF"); }
        }
    }

    let header: AITLFileHeader = serde_json::from_str(&String::from_utf8(header).unwrap()).unwrap();

    let mut demo = Vec::new();
    for _ in 0..header_size {
        match data.next() {
            Some(item) => {
                demo.push(item);
            }
            None => { }
        }
    }

    let data = AITLFile{ header: header, demo};

    Ok(data)
}

pub fn save_AITL_file(path: &str, demo: &[u8], header: AITLFileHeader) -> Result<()> {

    let data = serde_json::to_string(&header).unwrap();
    let mut header = data.as_bytes().to_vec();
    let mut header_size = format!("{}\0",header.len()).as_bytes().to_vec();
    let mut content = demo.to_vec();

    let mut save_buffer = Vec::new();
    save_buffer.append(&mut header_size);
    save_buffer.append(&mut header);
    save_buffer.append(&mut content);

    fs::write(path, save_buffer).unwrap();

    Ok(())
}

pub fn extract_AITL_file(input_path: &str, output_path: &str) -> Result<()> {

    let data = open_AITL_file(input_path).unwrap();

    let label_filename = format!("{output_path}/{}", data.header.label_filename);
    
    let label = data.header.label;
    let demo_filename = label.clone().demo_file;

    let label = toml::to_string_pretty(&label)?;

    let demo_data = data.demo;
    fs::write(&demo_filename, &demo_data)?;

    fs::write(&label_filename, &label)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITLFileHeader {
    pub label: Label,
    pub label_filename: String,
}

#[derive(Debug, Clone)]
pub struct AITLFile {
    pub header: AITLFileHeader,
    pub demo: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub demo_file: String,
    pub cheater: bool,
    pub cheater_steam_id: Vec<String>,
}