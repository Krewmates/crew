//! Comandos globais de atalho (crew status, crew up, crew down)
//! MVP: Stubs para compilar. Implementação completa vem depois.

use crate::error::CrewResult;

/// Mostra status unificado de Git + Docker (MVP stub)
pub async fn status() -> CrewResult<()> {
    println!("📊 crew status - TODO: implementar com tokio::join!");
    Ok(())
}

/// Menu interativo para iniciar containers seletivamente (MVP stub)
pub async fn up() -> CrewResult<()> {
    println!("⬆️  crew up - TODO: implementar menu Docker");
    Ok(())
}

/// Menu interativo para parar containers seletivamente (MVP stub)
pub async fn down() -> CrewResult<()> {
    println!("⬇️  crew down - TODO: implementar menu Docker");
    Ok(())
}
