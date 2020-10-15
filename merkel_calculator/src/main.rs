fn count_size(h: u128) -> (u16, u128) {
    if h == 1 {
        println!("1 lvl, 1 hash, 32 size");
        return (1, 32);
    } else {
        let top_data = count_size(h / 2);
        println!("{} lvl, {} hashes, {} size", top_data.0 + 1, h, h * 32);
        (top_data.0 + 1, h * 32)
    }
}

fn main() {
    let block_size = 2; // B
    let file_size = 1; // GB
    let file_size_b: u128 = 1 * 1073741824;

    let file_size_b: u128 = 7;


    println!("\n\nFile size is {} GB   or {} bytes\n\n", file_size, file_size_b);
    print!("Ill create {} full blocks  ", file_size_b / block_size);
    if file_size_b % block_size > 0 {
        println!("and one short block with {} bytes hashed\n", file_size_b % block_size);
    } else {
        println!("\n");
    }
    println!("**********************************");
    let total_hashes_size = count_size(file_size_b / block_size).1;
    println!("**********************************\n\n");

    let total_hashes_size_bytes_float: f64 = total_hashes_size as f64 / 1073741824.0;
    println!("Total hashes size: {} or {:.3} GB\n\n", total_hashes_size, total_hashes_size_bytes_float);
}
