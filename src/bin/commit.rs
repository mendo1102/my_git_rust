use std::env;
use std::fs;
use std::io::Write;
use chrono::Local;
use my_git_rust::{read_index, write_object};


fn main() {
    let args: Vec<String> = env::args().collect();
        let message_index = args.iter().position(|x| x == "-m");
    let message = if let Some(i) = message_index {
        args.get(i + 1)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "メッセージが指定されていません".to_string())
    } else {
        "デフォルトのコミットメッセージ".to_string()
    };

    let entries = read_index(); // indexを読み込む
    for entry in &entries {  // 参照でイテレートする(&を追加)
        println!("ファイル: {} ハッシュ: {}", entry.path, entry.hash);
    }

    // tree オブジェクトを生成
    let mut tree_data = Vec::new();
    for entry in &entries {  // 参照でイテレート
        let line = format!("{} {}\n", entry.hash, entry.path);
        tree_data.extend_from_slice(line.as_bytes());
    }
    let tree_hash = write_object(&tree_data);
    println!("tree: {}", tree_hash);

     // 現在時刻を取得して authorの行を生成
    let timestamp = Local::now().timestamp();
    // ↓ 今確認用で、固定
    let author = format!("mendo <mendo@example.com> {}", timestamp);

    // コミットデータを生成
    let commit_data = format!(
        "tree {}\nauthor {}\n\n{}\n",
        tree_hash,
        author,
        message
    );
    // コミットオブジェクトを書き込む
    let commit_hash = write_object(commit_data.as_bytes());

    // HEADに最新のコミットハッシュを書き込む
    let mut head = fs::File::create(".mygit/HEAD").expect("失敗: HEAD作成");
    writeln!(head, "{}", commit_hash).expect("失敗: HEAD書込");

    println!("commit: {}", commit_hash);
}
