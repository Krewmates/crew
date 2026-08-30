//! Comandos específicos de Docker
//! MVP: Stubs para compilar. Implementação completa vem depois.

use crate::error::CrewResult;

/// Lista containers rodando (MVP stub)
pub async fn list_containers() -> CrewResult<()> {
    println!("📦 docker ps - TODO: implementar listagem via bollard");
    Ok(())
}

/// Mostra logs de um container (MVP stub)
pub async fn show_logs(container: &str) -> CrewResult<()> {
    println!(
        "📋 docker logs {} - TODO: implementar tail de logs",
        container
    );
    Ok(())
}

/// Mata um container (MVP stub)
pub async fn kill_container(container: &str) -> CrewResult<()> {
    println!(
        "💀 docker kill {} - TODO: implementar via bollard",
        container
    );
    Ok(())
}
