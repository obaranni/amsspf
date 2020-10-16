use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;

use std::io::Read;
use std::io::Write;
use std::path::Path;
use serde_json;
extern crate config;
use crate::Settings;

impl super::Read for super::FilesList {
    fn read(&mut self, settings: &Settings) -> Result<(), Error> {   
        let fl_path = Path::new(&settings.storage.fl_path);
        let fl_name = Path::new(&settings.storage.fl_name); 

        match File::open(fl_path.join(fl_name)) {
            Ok(mut file) => {
                let mut files_json = String::new();
                file.read_to_string(&mut files_json).unwrap();
                self.current_files = serde_json::from_str(&files_json).unwrap();
                Ok(())
            },
            Err(error) => match error.kind() {
                // check fl existing
                ErrorKind::NotFound =>   {
                    fs::create_dir_all(fl_path)?;
                    let mut fc = File::create(fl_path.join(fl_name))?;
                    fc.write("[]".as_bytes()).unwrap();
                    Ok(())
                },
                other_error => {
                    println!("Problem opening the file: {:?}", other_error);
                    Err(other_error.into())
                }
            },
        }
    }
}
