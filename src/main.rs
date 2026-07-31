mod cli;
mod docker;

use clap::Parser;
use cli::{Cli, Command, ListCommands};
use docker::DockerClient;

#[tokio::main]
async fn main() {
    // Parse the CLI input
    let args: Cli = Cli::parse();
    let docker_client = DockerClient::new();

    // Handle the commands
    match args.command {
        Command::List { list_command } => match list_command {
            ListCommands::Containers { all } => {
                println!("Printing containers:");
                match docker_client.list_containers(all).await {
                    Ok(containers) => {
                        for container in containers {
                            println!(
                                "{}\t{}\t{}",
                                container.id.unwrap_or_default(),
                                container.names.unwrap_or_default().join(","),
                                container.status.unwrap_or_default()
                            );
                        }
                    }
                    Err(e) => eprintln!("Error listing containers: {}", e),
                }
            }
        },
    }
}
