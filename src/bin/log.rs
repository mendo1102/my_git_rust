use std::fs;
// use std::path::Path;
use chrono::{Local, TimeZone}; 
use my_git_rust::read_object;

fn main() {
    let mut commit_hash = fs::read_to_string(".mygit/HEAD")
        .expect("失敗: HEAD読み込み失敗")
        .trim()
        .to_string();

    while !commit_hash.is_empty() {
        let content = read_object(&commit_hash).expect("オブジェクト読み込み失敗");
        let text = String::from_utf8_lossy(&content);

        let mut lines = text.lines();
        let mut parent = String::new();
        let mut author_line = String::new();
        let mut message = String::new();

        while let Some(line) = lines.next() {
            if line.starts_with("parent ") {
                parent = line[7..].to_string();
            } else if line.starts_with("author ") {
                author_line = line.to_string();
            } else if line.trim().is_empty() {
                message = lines.collect::<Vec<_>>().join("\n");
                break;
            }
        }

        // author 日付を整形
        let parts: Vec<&str> = author_line.split_whitespace().collect();
        let timestamp: i64 = parts.last().unwrap_or(&"0").parse().unwrap_or(0);        
        // let datetime = DateTime::<Local>::from(
        //     NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or_default()
        // );

        let datetime = Local.timestamp_opt(timestamp, 0).single().unwrap_or_else(|| {
        eprintln!("失敗:日付の変換");
        Local::now() // fallback
        });
        println!("commit {}", commit_hash);
        println!("Author: {} <{}>", parts[1], parts[2]);
        println!("Date:   {}", datetime.format("%Y-%m-%d %H:%M:%S"));
        println!("\n    {}\n", message);

        commit_hash = parent; // 親コミットをたどる
    }
}
