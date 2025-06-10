use std::env;
use std::fs::{self, File};
use std::io::Write;
use sha1::{Sha1, Digest};
use serde::Serialize;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde_json;

#[derive(Serialize)]
struct IndexEntry {
    path: String,
    hash: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("確認: update_index <file>");
        std::process::exit(1);
    }
    let path = &args[1];
    let content = fs::read(path).expect("読み込み失敗");

    let header = format!("blob {}\0", content.len());
    let full = [header.as_bytes(), &content].concat();
    let mut hasher = Sha1::new();
    hasher.update(&full);
    let hash = hex::encode(hasher.finalize());

    let entry = IndexEntry {
        path: path.to_string(),
        hash: hash.clone(), // clone しておくとこの後使うため
    };

    let json = serde_json::to_string_pretty(&vec![entry]).unwrap();

    fs::create_dir_all(".mygit").unwrap();
    let mut file = File::create(".mygit/index").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    let dir = &hash[..2];
    let file = &hash[2..];
    let object_path = format!(".mygit/objects/{}/{}", dir, file);
    fs::create_dir_all(format!(".mygit/objects/{}", dir)).unwrap();

    let object_file = File::create(&object_path).unwrap();
    let mut encoder = ZlibEncoder::new(object_file, Compression::default());
    encoder.write_all(&full).unwrap();
    encoder.finish().unwrap();
    println!("Indexed {}", path);
}
