use crate::storage::hash_tree;
use rand::{thread_rng, Rng};
use crate::Settings;

impl super::Check for super::FilesList {
    fn check(&mut self, settings: &Settings) {
        let mut rng = thread_rng();
        let files = &self.current_files;
    
        for file in files {
            let block_numb = rng.gen_range(0, file.block_count) as usize;
    
            println!("Block to check {} in file {}", block_numb, file.name);
            hash_tree::verify_file(&file, block_numb, settings);
        }
    }   
}