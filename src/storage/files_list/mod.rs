mod read; 
mod update;
mod check;

pub use read::*;
pub use update::*;
pub use check::*;

use serde::Serialize;
use serde::Deserialize;

use std::io::Error;
use crate::Settings;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataV1 {
    pub hash: String, // sha256
    pub block_size: usize,
    pub author_name: String,
    pub author_pub_key: String,
    pub host_pub_key: String,
    pub host_download_link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Version {
    V1(DataV1)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileStruct {
    pub name: String,
    pub size: u64,
    pub block_count: u64,
    pub version: u8,
    
    pub data: Version,
}

pub struct FilesList {
    pub current_files: Vec<FileStruct>,
}

pub trait Read {
    fn read(&mut self, settings: &Settings) -> Result<(), Error>;
}

pub trait Check {
    fn check(&mut self, settings: &Settings);
}

pub trait Update {
    fn update(&mut self, settings: &Settings) -> std::io::Result<()>;
}