use clap::Parser;

// Command Line Arguments
#[derive(Parser)]
#[command(
    name = "rdcm",
    about = "Manage Docker containers using Rust and Docker API"
)]
pub struct Cli {}

//_____________________________________________________________________________
