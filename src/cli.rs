use clap::{Parser, Subcommand};

//_____________________________________________________________________________

// Cli Arguments that will be parsed by `clap`

#[derive(Parser)]
#[command(name = "rdcm", about = "Rust Docker Container Manager")]
pub struct Cli {
    // This is how you add the sub-command functionality to the `rdcm`
    // command.
    //
    // `command: Command` ensures that only valid sub-commands
    // from the `Command` enum can be used.
    #[command(subcommand)]
    pub command: Command,
}

//_____________________________________________________________________________

// A list of the valid subcommands
// E.g. The `list` subcommand
// Usage:
// rdcm list

#[derive(Subcommand)]
pub enum Command {
    // This is `rdcm list`
    // `list` is a subcommand that also has its own list of valid subcommands

    List {
        // These are sub-commands for the sub-command called `list`
        #[command(subcommand)]
        list_command: ListCommands,
    },
}

//_____________________________________________________________________________

// This is a list of valid subcommands for the list subcommand

#[derive(Subcommand)]
pub enum ListCommands {
    
    // NOTE: If you can add a descripion of what a command does 
    // by using `///` followed by a comment

    /// List containers
    Containers,
    Images,
    Start, // This will have a positional argument <container_id>
    Stop,  // This will have a positional argument <container_id>
    Pull,  // This will have a positional argument <container_id>
}

//_____________________________________________________________________________
