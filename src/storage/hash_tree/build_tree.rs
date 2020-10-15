use rocksdb::{Options, DB};
use sha2::{Digest, Sha256};
use sha2;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::str;

use crate::storage::files_list::{DataV1, FileStruct, Version};
use crate::Settings;


fn generate_lvl(db: &DB, start_from: u64, hash_count: u64) -> Vec<u8> {
    let mut j = hash_count;
    let mut i = start_from;
    let mut hash= Vec::new();

    while i < hash_count {
        
        let mut h1 = db.get(i.to_be_bytes()).unwrap().unwrap();
        // println!("H1 {:}", hex::encode(&h1));
        
        if i + 1 < hash_count {
            let h2 = db.get((i + 1).to_be_bytes()).unwrap().unwrap();
            // print!("H2 {:}", hex::encode(&h2));
            h1.extend(&h2);
            hash = Sha256::digest(&h1).to_vec();

        } else {
            // ---------h1.extend(&h1);
            // print!("H2 {:}", hex::encode(&h1));
            hash = Sha256::digest(&h1).to_vec();
        }
        // println!("    => {:}\n", hex::encode(&hash));

        db.put(j.to_be_bytes(), &hash).unwrap();
        j += 1;
        i += 2;
    }

    if j - hash_count == 1 {
        println!("Root hash found");
        return hash;
    }
    generate_lvl(&db, hash_count, j)
}

fn generate_base_lvl(file: &mut File, settings: &Settings, proof_dir_path: &Path) -> Vec<u8> {
    let block_size = settings.storage.block_size;
    let mut iterator: u64 = 0;
    let mut buffer = vec![0u8; block_size];
    let mut bytes: usize;
    let db = DB::open_default(&proof_dir_path).unwrap();

    loop {
        bytes = file.read(&mut buffer).unwrap();

        if bytes < block_size || (file.metadata().unwrap().len() as usize) <= block_size {
            if bytes > 0 {
                let hash = Sha256::digest(&mut buffer[..bytes]);
                // println!("BHf{} {:}    bytes {}", iterator, hex::encode(&hash), bytes);
                db.put(iterator.to_be_bytes(), hash).unwrap();
                if iterator == 0 {
                    return hash.to_vec();
                }
                iterator += 1;
            }
            break;
        }

        let hash = Sha256::digest(&mut buffer[..bytes]);
        db.put(iterator.to_be_bytes(), hash).unwrap();

        // println!("BHr{} {:}", iterator, hex::encode(&hash));
        iterator += 1;
    }
    // println!("");
    let top_hash = generate_lvl(&db,0, iterator);

    let _ = DB::destroy(&Options::default(), proof_dir_path);
    top_hash
}

pub fn build(file_path: &Path, settings: &Settings) -> io::Result<FileStruct> {
    let f = File::open(&file_path)?;
    let metadata = f.metadata()?;
    let block_size = settings.storage.block_size;
    let blocks_total = ((metadata.len() as usize) / block_size)
        + if (metadata.len() as usize) % block_size > 0 {
            1
        } else {
            0
        };

    println!(" ******  File Analysis  ******");
    println!(" * Name: {}\n * Size: {:?} bytes\n * Block size: {}\n * Blocks count: {}\n * Tail block size: {}", file_path.to_str().unwrap(), metadata.len(), block_size, blocks_total, (metadata.len() as usize) % block_size);
    println!(" *****************************\n\n");

    let mut file_to_load = File::open(&file_path)?;

    let proof_dir_path = Path::new(&settings.storage.proof_path).join(&file_path.file_name().unwrap());

    let res = fs::remove_dir_all(&proof_dir_path);

    match res {
        Ok(()) => println!("Dir removed"),
        Err(res) => println!("Error {}", res),
    }

    let top_hash = generate_base_lvl(&mut file_to_load, &settings, &proof_dir_path);

    let file_obj = FileStruct {
        name: String::from(file_path.file_name().unwrap().to_str().unwrap()),
        size: metadata.len(),
        block_count: blocks_total as u64,
        version: 1,

        data: Version::V1(DataV1 {
            hash: hex::encode(&top_hash), // sha256
            block_size: block_size,
            author_name: String::from("Some Author"),
            author_pub_key: String::from("Some Author"),
            host_pub_key: String::from("Some Author"),
            host_download_link: String::from("Some Author"),
        }),
    };

    Ok(file_obj)
}

pub fn read(file_name: &str) {
    let mut iterator: u64 = 0;

    let tt = String::from("./proof_of_storage/");
    let proof_dir_path = Path::new(&tt).join(file_name);

    let db = DB::open_default(&proof_dir_path).unwrap();

    while iterator < 3 {
        match db.get(iterator.to_be_bytes()) {
            Ok(Some(value)) => {
                println!("{:?} {:}", value, hex::encode(&value));
            }
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }

        iterator += 1;
    }

    let _ = DB::destroy(&Options::default(), proof_dir_path);
}
