use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexEntry {
    pub path: String,
    pub hash: String,
}

/// index を読み込む処理
pub fn read_index() -> Vec<IndexEntry> {
    let data = fs::read_to_string(".mygit/index").expect("index読み込み失敗");
    serde_json::from_str(&data).expect("index parse失敗")
}

/// オブジェクトを書き込む + hash を返す処理
pub fn write_object(data: &[u8]) -> String {
    // let mut hasher = Sha1::new();
    // hasher.update(data);
    let header = format!("blob {}\0", data.len());
    let full = [header.as_bytes(), data].concat();

    // SHA1でハッシュ計算
    let mut hasher = Sha1::new();
    hasher.update(&full);
    let hash = hex::encode(hasher.finalize());

    // オブジェクトのpath作成
    let dir = format!(".mygit/objects/{}", &hash[..2]);
    let path = format!("{}/{}", dir, &hash[2..]);
    fs::create_dir_all(&dir).unwrap();

    // Zlib圧縮して保存(これをしないとlogでt、展開の時にエラー)
    let file = File::create(&path).unwrap();
    let mut encoder = ZlibEncoder::new(file, Compression::default());
    encoder.write_all(&full).unwrap();
    encoder.finish().unwrap();

    hash
}

pub fn read_object(hash: &str) -> std::io::Result<Vec<u8>> {
    let dir = &hash[..2];
    let file = &hash[2..];
    let path = format!(".mygit/objects/{}/{}", dir, file);
    let data = fs::read(path)?;
    let mut decoder = flate2::read::ZlibDecoder::new(&data[..]);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)?;
    Ok(result)
}
