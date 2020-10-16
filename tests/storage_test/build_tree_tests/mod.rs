extern crate amsspf;

use amsspf::storage;
use amsspf::Network;
use amsspf::Storage;
use amsspf::Settings;

use std::path::Path;
use rocksdb::{Options, DB};

fn create_settings(block_size: usize) -> Settings {
    Settings {
        debug: true,
        storage: Storage {
            storage_path: String::from("./tests/storage_test/build_tree_tests/test_storage"),
            fl_path: String::from(""),
            fl_name: String::from(""),
            proof_path: String::from("./tests/storage_test/build_tree_tests/proof_of_storage"),
            block_size: block_size,
        },
        network: Network {
        }
    }
}

#[test]
pub fn one_block_1024_no_tail() {
    let settings = create_settings(1024);
    let file_path = Path::new(&settings.storage.storage_path).join("type_b_1.txt");
    let proof_dir_path = Path::new(&settings.storage.proof_path).join("type_b_1.txt");

    storage::build(&file_path, &settings).unwrap();

    let db = DB::open_default(&proof_dir_path).unwrap();


    assert_eq!(db.get((0u64).to_be_bytes()).unwrap().unwrap(), hex::decode("17907d65adc014e0144769fad469a0e2cc290621cce5d6ef5c63a4a80b5fd2c6").unwrap());







    let _ = DB::destroy(&Options::default(), &proof_dir_path);
}