// https://engineering.mercari.com/blog/entry/2015-09-14-175300/を参考
use std::env;
use std::fs::File;
use std::io::{self, Read};
use flate2::read::ZlibDecoder;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cat-file <object file>");
        std::process::exit(1);
    }

    let path = &args[1];
    let file = File::open(path)?;
    let mut decoder = ZlibDecoder::new(file);

    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    // Gitオブジェクトは "blob 14\0" のようなヘッダー付きにする
    if let Some(null_pos) = buffer.iter().position(|&b| b == 0) {
        let content = &buffer[null_pos + 1..]; // ヘッダーをスキップさせる
        print!("{}", String::from_utf8_lossy(content));
    } else {
        eprintln!("git objectのfmtが不正");
    }

    Ok(())
}
