mod build_tree;
mod verify_tree;

pub use build_tree::*;
pub use verify_tree::*;
use std::path::{PathBuf};

use crate::storage::files_list::FileStruct;

// #[derive(Serialize, Deserialize, Clone)]
pub struct VerifyPack {
    pub block: Vec<u8>,
    pub block_numb: usize,

    pub file_meta: FileStruct,
    pub storage_path: PathBuf,
}