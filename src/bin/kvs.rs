use std::{fs::{create_dir, exists}, process::exit};

use clap::{Parser, Subcommand};
use kvs::{KvStore, KvsError};

#[derive(Subcommand)]
enum KvsCommand {
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },

    #[command(name = "rm")]
    Remove {
        key: String,
    },
}

#[derive(Parser)]
#[command(author, version)]
struct KvsConfig {
    #[command(subcommand)]
    command: KvsCommand,
}

fn guarded_main() -> kvs::Result<()> {
    // There could be more errors here (i.e. a file was created with the name "store")
    // Buttttt... I don't care.
    if let Err(_error) = exists("store") {
        return Err(KvsError::Unknown);
    }

    let mut store = KvStore::open("store")?;
    let parser = KvsConfig::parse();

    match parser.command {
        KvsCommand::Set { key, value } => {
            return store.set(key, value);
        }
        KvsCommand::Get { key } => {
            return Ok(());
        }
        KvsCommand::Remove { key } => {
            return Ok(());
        }
    }
}

fn main() {
    let result = guarded_main();
    let mut exit_code = 0;

    if let Err(error) = result {
        println!("{:?}", error);
        exit_code = 1;
    }

    exit(exit_code);
}
