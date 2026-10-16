use std::{env::current_dir, fs::{create_dir, exists}, io, process::exit};

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
    let mut store = KvStore::open(current_dir()?)?;
    let parser = KvsConfig::parse();

    match parser.command {
        KvsCommand::Set { key, value } => {
            return store.set(key, value);
        }
        KvsCommand::Get { key } => {
            let string = store.get(key)?;
            if let Some(string) = string {
                println!("{string}");
            }
            else {
                println!("Key not found");
            }
        }
        KvsCommand::Remove { key } => {
            let result = store.remove(key);
            return result.inspect_err(|error| {
                match error {
                    KvsError::BadKey => { println!("Key not found"); }
                    _ => { /* ... */ }
                }
            });
        }
    }

    Ok(())
}

fn main() {
    let result = guarded_main();
    let mut exit_code = 0;

    if let Err(error) = result {
        match error {
            // NOTE: Why does the same error with two different functions have different
            // results??
            // HACK: BadKey will be handled in guarded_main.
            KvsError::BadKey => { /* Shut up */ }
            _ => { println!("{:?}", error); }
        }

        exit_code = 1;
    }

    exit(exit_code);
}
