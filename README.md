# my_git_rust
　これは、Rust言語を使って簡易版のgitを構築したもの<br>
 　※単一ファイルのみ対応中(まだまだ修正しています)
  
**[参考のURL]**
<br>
[Gitの内側 - Gitオブジェクト](https://git-scm.com/book/ja/v2/Git%E3%81%AE%E5%86%85%E5%81%B4-Git%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88)
<br>
[Gitのつくりかた](https://engineering.mercari.com/blog/entry/2015-09-14-175300/)
<br>
(すごく勉強させていただきました。。。)

## Overview

- `add.rs` ......git add 
- `commit.rs` ...git commit
- `log.rs` ......git log
- `status.rs` ...git status
- `diff.rs` .....git diff

## Directory Structure
下記を.gitの代わりに、.mygitとして生成
```
.mygit/          # .gitではなく今回のプロジェクト用に、./mygitを生成
├── objects/     # blobs and commits (zlib-compressed)
├── index        # JSON-based index file
└── HEAD         # Hash of the latest commit
```

## Usage

ディレクトリ構成
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

```bash
(例) sample.txt
cargo run --bin add sample.txt
cargo run --bin status
cargo run --bin diff sample.txt
cargo run --bin commit -- -m "initial commit"
cargo run --bin log
```
