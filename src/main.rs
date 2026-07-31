// SECTION: Modules


mod cli;
mod docker;

use cli::{Cli, Command};

//_____________________________________________________________________________

// SECTION: External Dependencies

use clap::Parser;

//_____________________________________________________________________________

fn main() {
    let args = Cli::parse();

    match args.command {
       // `list_command: _` tells Rust that I am not using the value from
       // the `list_command` field inside the match arm
       Command::List { list_command: _ } => {
            println!("Listing all containers...");
       }
    }
}
