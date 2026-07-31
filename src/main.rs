mod cli;
use cli::{Cli, Command, ListCommands};

mod docker;
use docker::DockerClient;

use clap::Parser;

fn main() {
    // Parse the CLI input
    let args: Cli = Cli::parse();

    let docker_client = DockerClient::new();

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
