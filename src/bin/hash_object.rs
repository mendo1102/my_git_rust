use std::env;
use std::fs;
use sha1::{Sha1, Digest};
use libflate::zlib::Encoder;
use std::io::Write;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("確認: hash_object <file>");
        std::process::exit(1);
    }

    let path = &args[1];
    let content = fs::read(path).expect("失敗： to read file");

    let header = format!("blob {}\0", content.len());
    let full = [header.as_bytes(), &content].concat();

    let mut hasher = Sha1::new();
    hasher.update(&full);
    let hash = hasher.finalize();
    let hex_hash = hex::encode(hash);
    println!("{}", hex_hash);

    let dir = format!(".mygit/objects/{}", &hex_hash[..2]);
    let file_path = format!("{}/{}", dir, &hex_hash[2..]);

    fs::create_dir_all(&dir).expect("失敗： create directory");

    let file = File::create(file_path).expect("失敗： create object file");
    let mut encoder = Encoder::new(file).expect("失敗： create encoder");
    encoder.write_all(&full).expect("失敗： write object");
    encoder.finish().into_result().expect("失敗： finish compression");
}
