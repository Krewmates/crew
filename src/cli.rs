use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "crew")]
#[command(about = "Developer experience CLI for Git + Docker orchestration")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Global shortcuts
    #[command(subcommand)]
    Global(GlobalCommands),

    /// Git interactive menu
    Git(GitCommand),

    /// Docker operations
    #[command(subcommand)]
    Docker(DockerCommands),
}

#[derive(Subcommand, Debug)]
pub enum GlobalCommands {
    Status,
    Up,
    Down,
}

#[derive(Parser, Debug)]
pub struct GitCommand {
    // MVP: Sem subcomandos. `crew git` abre o menu.
}

#[derive(Subcommand, Debug)]
pub enum DockerCommands {
    Ls,
    Logs { container: String },
    Kill { container: String },
}
