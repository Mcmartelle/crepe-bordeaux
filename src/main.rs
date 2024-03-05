use anyhow::Result;
use atty::Stream;
use clap::{Parser, Subcommand};
use crepe_bordeaux::{clear, clear_all, copy, dump, get_register_dir, list, paste};
use std::io::{stdin, Read};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Print piped in contents and where it was saved
    #[clap(long)]
    verbose: bool,

    /// Optional register name (Will save to .txt file, not the system clipboard)
    register: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists registers
    List,
    /// Displays register directory
    Dir,
    /// Outputs contents of all registers
    Dump,
    /// Clears specified register
    Clear,
    /// Clears all registers
    ClearAll,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => {
            list()?;
            return Ok(());
        }
        Some(Commands::Dir) => {
            println!("{}", get_register_dir()?.to_str().unwrap());
            return Ok(());
        }
        Some(Commands::Dump) => {
            dump()?;
            return Ok(());
        }
        Some(Commands::Clear) => {
            clear(cli.register.as_deref())?;
            return Ok(());
        }
        Some(Commands::ClearAll) => {
            clear_all()?;
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
                false => copy(content, cli.register.as_deref(), cli.verbose)?,
            }
        }
        // Handle `cb ...` cases
        false => paste(cli.register.as_deref())?,
    }

    Ok(())
}
