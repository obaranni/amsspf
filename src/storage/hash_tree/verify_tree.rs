use std::path::{Path};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use rocksdb::{Options, DB};
use sha2::{Digest, Sha256};

use crate::storage::files_list::FileStruct;
use crate::storage::files_list::DataV1;
use crate::storage::files_list::Version;
use crate::storage::hash_tree;
use crate::storage::hash_tree::VerifyPack;
use crate::Settings;


fn get_file_size(file_path: &Path) -> u64 {
    let f = File::open(&file_path).unwrap();
    let metadata = f.metadata().unwrap();

    metadata.len()
}

pub fn get_file_block(file: &FileStruct, block_numb: usize) -> io::Result<Vec<u8>> {
    let storage = Path::new("./storage").join(&file.name);
    let block_size = match &file.data {
        Version::V1(data) => {
            data.block_size
        }
    };

    let mut f = File::open(&storage)?;
    let mut buffer = vec![0u8; block_size];


    f.seek(SeekFrom::Start((block_numb * block_size) as u64))?;
    let bytes = f.read(&mut buffer).unwrap();

    if block_numb == ((file.block_count - 1) as usize) && bytes < block_size {
        let bytes = (file.size as usize) % block_size;
        Ok(buffer[..bytes].to_vec())
    } else {
        Ok(buffer[..bytes].to_vec())
    }
}

pub fn get_hashes_way(verify_info : &VerifyPack, block_numb: usize, settings: &Settings) -> io::Result<Vec<Vec<u8>>> {
    let proof_dir_path = Path::new(&settings.storage.proof_path).join(&verify_info.file_meta.name);
    let file_size = get_file_size(&verify_info.storage_path);
    let block_size = match &verify_info.file_meta.data {
        Version::V1(d) => {
            d.block_size
        }
    };
    let blocks_total = ((file_size as usize) / block_size)
    + if (file_size as usize) % block_size > 0 {
        1
    } else {
        0
    };

    if block_numb > blocks_total {
        println!("Error");
    }

    let mut way = Vec::new();
    // println!("Proof folder {:?}", proof_dir_path.as_os_str());
    let db = DB::open_default(&proof_dir_path).unwrap();

    let mut offset = 0;
    let mut hash_position = block_numb;
    let mut hashes_on_lvl = blocks_total;

    while hashes_on_lvl > 1 {

        // println!("Pair #{}", hash_position / 2);
        // println!("Hash position: {} , hashes on lvl: {}", hash_position, hashes_on_lvl);
        
        if hash_position % 2  != 0 { // Take left hash
            hash_position -= 1;
        } else if (hash_position + 1) == hashes_on_lvl { // Self hash
            // println!("Self hash {}", hash_position);

        } else { // Take right hash
            hash_position += 1;
        }

        match db.get((offset + hash_position).to_be_bytes()) {
            Ok(Some(value)) => {
                way.push(value);
            }
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }


        hash_position = hash_position / 2;
        offset += hashes_on_lvl;
        hashes_on_lvl = (hashes_on_lvl / 2) + (hashes_on_lvl % 2);
    }

    let _ = DB::destroy(&Options::default(), proof_dir_path);

    Ok(way)
}

pub fn verify_v1(verify_info: &VerifyPack, file_data: &DataV1, hashes_way: &Vec<Vec<u8>>) -> bool {
    let mut hashes_on_lvl = verify_info.file_meta.block_count as usize;
    let mut hash_position = verify_info.block_numb;
    let mut hashes_iterator = 0;
    let mut result_hash: Vec<u8> = Sha256::digest(&verify_info.block).to_vec();

    while hashes_on_lvl > 1 {
        
        let mut hash_in_way = hashes_way[hashes_iterator].clone();

        if hash_position % 2  != 0 {
            hash_position -= 1;

            // println!("Left with right (previous)");
            // println!("H1 {}", hex::encode(&hash_in_way));
            // print!("H2 {}", hex::encode(&result_hash));
            hash_in_way.append(&mut result_hash);
            result_hash = Sha256::digest(&hash_in_way).to_vec();
            // println!("  => {}", hex::encode(&result_hash));

        } else if (hash_position + 1) == hashes_on_lvl {
            // println!("Self");
            // println!("H1 {}", hex::encode(&result_hash));
            // print!("H2 {}", hex::encode(&result_hash));
            result_hash = Sha256::digest(&hash_in_way).to_vec();
            // println!("  => {}", hex::encode(&result_hash));
        } else {
            // println!("Left (previous) with right");
            // println!("H1 {}", hex::encode(&result_hash));
            // print!("H2 {}", hex::encode(&hash_in_way));   
            result_hash.append(&mut hash_in_way);
            result_hash = Sha256::digest(&result_hash).to_vec();
            // println!("  => {}", hex::encode(&result_hash));
        }

        hashes_iterator += 1;
        hash_position = hash_position / 2;
        hashes_on_lvl = (hashes_on_lvl / 2) + (hashes_on_lvl % 2);
    }

    // println!("{:?} \n{:?}", hex::decode(&file_data.hash).unwrap(), result_hash);
    if hex::decode(&file_data.hash).unwrap() == result_hash {
        println!("File verified!");
        return true
    }
    panic!("Bad proof!");
    // false
}

pub fn verify_file(file: &FileStruct, block_numb: usize, settings: &Settings) {
    let verify_data = VerifyPack {
        block: get_file_block(&file, block_numb).unwrap(),
        block_numb,
        file_meta: file.clone(),
        storage_path: Path::new(&settings.storage.storage_path).join(&file.name),
    };

    let hashes_way = hash_tree::get_hashes_way(&verify_data, block_numb, settings).unwrap();


    
    match &verify_data.file_meta.data {
        Version::V1(data) => {
            verify_v1(&verify_data, &data, &hashes_way);
        },
    }
}
