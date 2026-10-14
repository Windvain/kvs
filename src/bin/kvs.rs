use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Subcommand)]
enum KvsCommand {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

#[derive(Parser)]
#[command(author, version)]
struct KvsConfig {
    #[command(subcommand)]
    command: KvsCommand,
}

fn main() {
    let parser = KvsConfig::parse();
    match parser.command {
        KvsCommand::Set { key, value } => {
            eprintln!("unimplemented");
        }
        KvsCommand::Get { key } => {
            eprintln!("unimplemented");
        }
        KvsCommand::Remove { key } => {
            eprintln!("unimplemented");
        }
    }

    exit(1);
}
