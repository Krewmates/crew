mod cli;
mod commands;
mod config;
mod docker;
mod error;
mod git;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status => {
            println!("Executando: crew status...");
        }
        Commands::Up => {
            println!("Executando: crew up...");
        }
    }
}
