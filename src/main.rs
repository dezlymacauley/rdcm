// SECTION: Modules

mod cli;
use cli::Cli;

mod docker;

//_____________________________________________________________________________

// SECTION: External Dependencies

use clap::Parser;

//_____________________________________________________________________________

fn main() {
    Cli::parse();
}
