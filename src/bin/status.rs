use std::fs;

#[derive(serde::Deserialize)]
struct IndexEntry {
    path: String,
    hash: String,
}

fn compute_blob_hash(path: &str) -> String {
    // hash_object.rs のロジックを使ってSHA-1ハッシュを再計算している
    use sha1::{Sha1, Digest};
    let content = fs::read(path).expect("失敗: to read file");
    let header = format!("blob {}\0", content.len());
    let full = [header.as_bytes(), &content].concat();
    let mut hasher = Sha1::new();
    hasher.update(&full);
    hex::encode(hasher.finalize())
}

fn load_index(path: &str) -> Vec<IndexEntry> {
    let content = fs::read_to_string(path).expect("indexファイルが読み込めませんでした");
    serde_json::from_str(&content).expect("indexのパースに失敗しました")
}

fn main() {
    let index = load_index(".mygit/index");
    for entry in index {
        let current_hash = compute_blob_hash(&entry.path);
        if current_hash != entry.hash {
            println!("modified: {}", entry.path);
        }
    }
}
