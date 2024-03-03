use anyhow::Result;
use atty::Stream;
use clap::{Parser, Subcommand};
use crepe_bordeaux::{copy, paste};
use std::io::{stdin, Read};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional register name (Will save to .txt file, not the system clipboard)
    register: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists non-empty registers
    List,
    /// Displays register directory
    Dir,
    /// Outputs contents of all registers
    Dump,
    /// Clears all registers
    ClearAll,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => {
            println!("list, not implemented");
            return Ok(());
        }
        Some(Commands::Dir) => {
            println!("dir, not implemented");
            return Ok(());
        }
        Some(Commands::Dump) => {
            println!("dump, not implemented");
            return Ok(());
        }
        Some(Commands::ClearAll) => {
            println!("clear-all, not implemented");
            return Ok(());
        }
        None => {}
    }

    match atty::isnt(Stream::Stdin) {
        true => {
            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer).unwrap();
            let content = buffer.trim();
            match content.is_empty() {
                // Handle `echo '' | cb ...` cases
                true => paste(cli.register.as_deref())?,
                // Handle `echo 'foo' | cb ...` cases
                false => copy(content, cli.register.as_deref())?,
            }
        }
        // Handle `cb ...` cases
        false => paste(cli.register.as_deref())?,
    }

    Ok(())
}
