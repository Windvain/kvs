use std::process::exit;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum KvsCommand {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

#[derive(Parser)]
#[command(author, version)]
struct KvsParser {
    #[command(subcommand)]
    command: KvsCommand,
}

fn main() {
    let parser = KvsParser::parse();
    match parser.command {
        KvsCommand::Set { key, value } => {
            eprintln!("unimplemented");
        },
        KvsCommand::Get { key } => {
            eprintln!("unimplemented");
        },
        KvsCommand::Remove { key } => {
            eprintln!("unimplemented");
        }
    }

    exit(1);
}
