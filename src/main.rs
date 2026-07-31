mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommands};

fn main() {
    // Parse the CLI input
    let args: Cli = Cli::parse();

    // Handle the commands
    match args.command {
        Command::List { list_command } => match list_command {
            ListCommands::Containers { all } => {
                if all {
                    println!("Listing all containers");
                } else {
                    println!("Listing running containers");
                }
            }
        },
    }
}
