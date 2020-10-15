use std::fs;
use std::path::{Path};
use std::fs::OpenOptions;
use std::io::prelude::*;
use serde_json;

use crate::storage::hash_tree;
use crate::storage::files_list::FileStruct;
use crate::Settings;

fn find_in_current_list(current_files: &Vec<FileStruct>, file_name: &str) -> bool {
    for file in current_files {
        if file.name == file_name {
            return true;
        }
    }
    return false
}

pub fn save_files(files: &Vec<FileStruct>) -> std::io::Result<()> {
    
    let files_json = serde_json::ser::to_string_pretty(files).unwrap(); //json!(file);

    let file_options = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./fl/files_list.json");


    match file_options {
        Ok(mut file) => {
                match file.write(files_json.as_bytes()) {
                    Ok(_) => {
                        println!("New files added");
                        Ok(())
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        Ok(())
                    }
                }
        },
        Err(e) => {
            println!("Error: {}", e);
            Ok(())
        }
    }
}

impl super::Update for super::FilesList {
    fn update(&mut self, settings: &Settings) -> std::io::Result<()> {
        let files_count = self.current_files.len();
        let path_to_storage = Path::new(&settings.storage.storage_path);
    
        for entry in fs::read_dir(path_to_storage)? {
            let dir = entry?;
            let file_name: String = dir.file_name().into_string().unwrap();
            
            if find_in_current_list(&self.current_files, &file_name) == true {
                println!("File with name {} already added", file_name);
            } else {
                println!("Adding file with name {}", file_name);
                let new_file = hash_tree::build(&path_to_storage.join(file_name), settings).expect("err");
                self.current_files.push(new_file);
                
            }
        }
        if self.current_files.len() > files_count {
            save_files(&self.current_files).unwrap();
        }
        Ok(())
    }
}
