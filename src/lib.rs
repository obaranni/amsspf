pub mod storage;
mod settings;

extern crate serde_derive;

pub use settings::Network;
pub use settings::Storage;
pub use settings::Settings;
pub use crate::storage::*;

struct Context {
    settings: Settings,
    files_list: FilesList
}

fn get_context() -> Context {
    match Settings::new() {
        Ok(settings) => {
            Context {
                settings,
                files_list: FilesList {
                    current_files: Vec::new(),
                },
            }
        },
        Err(e) => {
            panic!(e.to_string());
        }
    }
}

pub fn run() {
    let context = get_context();
    let settings = &context.settings;
    let mut fl = context.files_list;

    fl.read(settings).unwrap();
    fl.check(settings);
    fl.update(settings).unwrap();   
}
