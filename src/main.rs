mod cli;
mod commands;
mod config;
mod docker;
mod error;
mod git;
mod ui;

use clap::Parser;
use cli::{Cli, Commands, GlobalCommands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Global(global_cmd) => match global_cmd {
            GlobalCommands::Status => {
                commands::global::status().await?;
            }
            GlobalCommands::Up => {
                commands::global::up().await?;
            }
            GlobalCommands::Down => {
                commands::global::down().await?;
            }
        },

        Commands::Git(_git_cmd) => {
            commands::git_cmds::run_git_menu().await?;
        }

        Commands::Docker(docker_cmd) => match docker_cmd {
            cli::DockerCommands::Ls => {
                commands::docker_cmds::list_containers().await?;
            }
            cli::DockerCommands::Logs { container } => {
                commands::docker_cmds::show_logs(&container).await?;
            }
            cli::DockerCommands::Kill { container } => {
                commands::docker_cmds::kill_container(&container).await?;
            }
        },
    }

    Ok(())
}
