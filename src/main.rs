use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mygit")]
#[command(about = "Rust製のGitクローンツール", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add {
        path: String,
    },
    Commit {
        #[arg(short, long)]
        message: String,
    },
    Log,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("init 実行");
            // my_git_rust::init(); // 既存の `init.rs` の処理呼び出し
        }
        Commands::Add { path } => {
            println!("add 実行: {}", path);
            // my_git_rust::add(path)
        }
        Commands::Commit { message } => {
            println!("commit 実行: {}", message);
            // my_git_rust::commit(message)
        }
        Commands::Log => {
            println!("log 実行");
            // my_git_rust::log()
        }
    }
}
