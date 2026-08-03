mod cli;
mod commands;
mod config;
mod docker;
mod error;
mod git;
mod ui;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status => {
            println!("Executando: crew status...");
            // Aqui você chamará a função do src/commands/status.rs no futuro
        }
        Commands::Up => {
            println!("Executando: crew up...");
        }
    }
}