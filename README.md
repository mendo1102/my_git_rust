# my_git_rust
　これは、Rust言語を使って簡易版のgitを構築したもの<br>
 　※単一ファイルのみ対応中(まだまだ修正しています)
  
**[参考のURL]**
<br>
[Gitの内側 - Gitオブジェクト](https://git-scm.com/book/ja/v2/Git%E3%81%AE%E5%86%85%E5%81%B4-Git%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88)
<br>
[Gitのつくりかた](https://engineering.mercari.com/blog/entry/2015-09-14-175300/)
<br>

## 概要

```
- `add.rs` ......git add 
- `commit.rs` ...git commit
- `log.rs` ......git log
- `status.rs` ...git status
- `diff.rs` .....git diff
```

## 構成

```
.
├── Cargo.lock
├── Cargo.toml
├── docker-compose.yml
├── README.md
├── .mygit
├── <任意でファイルを物理的に作成>
├── src
│   ├── bin
│   ├── lib.rs
│   └── main.rs
└── target
```

下記を.gitの代わりに、.mygitとして生成
```
.mygit/          # .gitではなく今回のプロジェクト用に、./mygitを生成
├── objects/     # ファイル内容（blob）やコミット情報を zlib 圧縮して格納するディレクトリ
├── index        # ステージングされたファイル情報を保持する JSON 形式のファイル
└── HEAD         # 最新のコミットハッシュを保持するファイル（ログの起点）
```



## Usage

ディレクトリ構成
ディレクトリの直下に
任意のリソースを作成
例
・sample.txt

```bash
(例) sample.txt
① cargo run --bin add sample.txt (git add と同様)
② sample.txt内の修正
③ cargo run --bin status (git statusと同様)
④ cargo run --bin diff sample.txt (git diffと同様)
cargo run --bin commit -- -m "内容変更" (git commit -m "コミットメッセージ")
cargo run --bin log (git logと同様)
```


