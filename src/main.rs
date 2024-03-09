use anyhow::Result;
use clap::{Parser, Subcommand};
use crepe_bordeaux::{clear, clear_all, copy, dump, get_register_dir, list, paste};
use std::io::{stdin, IsTerminal, Read};

#[derive(Parser)]
#[command(version, about)]
/// The cross-platform clipboard cli tool.
///
/// Copy to clipboard by piping in text `echo "foo" | cb`
/// and paste text to stdout by running `cb` on its own.
struct Cli {
    /// Print more detailed success and error messages
    #[clap(long)]
    verbose: bool,

    /// Optional register name (Will save to <register>.txt file, not the system clipboard)
    register: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists registers saved in the register directory.
    List,
    /// Displays the register directory path defined by Rust's std::env::temp_dir, override by setting the CB_DIR env variable.
    Dir,
    /// Outputs contents of all registers
    Dump,
    /// Clears the specified register. `cb clear` clears the system clipboard. `cb foo clear` clears the 'foo' register.
    Clear,
    /// Clears the system clipboard and all registers
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

    match std::io::stdin().is_terminal() {
        false => {
            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer).unwrap();
            let content = buffer.trim();
            match content.is_empty() {
                // Handle `echo '' | cb ...` cases
                true => paste(cli.register.as_deref(), cli.verbose)?,
                // Handle `echo 'foo' | cb ...` cases
                false => copy(content, cli.register.as_deref(), cli.verbose)?,
            }
        }
        // Handle `cb ...` cases
        true => paste(cli.register.as_deref(), cli.verbose)?,
    }

    Ok(())
}
