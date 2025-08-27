use std::env;
use std::process;
use xxhash_rust::xxh3::xxh3_64;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print!("Error: No URL provided");
        process::exit(1);
    }

    let db = match sled::open("./kv-npt") {
        Ok(db) => db,
        Err(e) => {
            print!("Error: Failed to open database: {}", e);
            process::exit(1);
        }
    };

    for url in args {
        let hash = xxh3_64(url.as_bytes());
        let hash_bytes = hash.to_be_bytes();

        match db.contains_key(&hash_bytes) {
            Ok(true) => {
                print!("1");
            }
            Ok(false) => {
                if let Err(e) = db.insert(&hash_bytes, &[1]) {
                    print!("Error: Failed to insert key: {}", e);
                    continue;
                }
                print!("0");
            }
            Err(e) => {
                print!("Error: Failed to check key existence: {}", e);
                continue;
            }
        }
    }

    if let Err(e) = db.flush() {
        print!("Error: Failed to flush database: {}", e);
    }
}