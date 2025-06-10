use std::env;
use std::fs;
use flate2::read::ZlibDecoder;
use std::io::Read;
use similar::{TextDiff, ChangeTag};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin diff <file>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let current = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("失敗: read file: {}", file_path));

    let index_content = fs::read_to_string(".mygit/index")
        .expect("Failed to read .mygit/index");
    let index: Vec<IndexEntry> = serde_json::from_str(&index_content)
        .expect("失敗: parse index");

    let entry = index.iter()
        .find(|e| e.path == *file_path)
        .expect("File not found in index");

    let hash = &entry.hash;
    let (dir, file) = hash.split_at(2);
    let object_path = format!(".mygit/objects/{}/{}", dir, file);

    let compressed = fs::read(&object_path)
        .unwrap_or_else(|_| panic!("失敗: read object: {}", object_path));

    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).expect("失敗： decompress blob");

    let content_start = decompressed.iter().position(|&b| b == 0).unwrap() + 1;
    let original = String::from_utf8_lossy(&decompressed[content_start..]);

    let diff = TextDiff::from_lines(&original[..], &current[..]);

    // 差分内容を出力
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal  => " ",
        };
        print!("{}{}", sign, change);
    }
}

#[derive(Debug, serde::Deserialize)]
struct IndexEntry {
    path: String,
    hash: String,
}
