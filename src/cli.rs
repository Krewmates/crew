use clap::{Parser, Subcommand};

#[derive(Parser)] // <-- Isso injeta o método parse() na struct
#[command(name = "crew", about = "A tripulação do terminal", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Mostra o painel cruzado de status (Git + Docker)
    Status,
    /// Abre o menu interativo para subir serviços seletivamente
    Up,
}