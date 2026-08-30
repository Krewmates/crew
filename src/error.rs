#![allow(dead_code)]

use std::io;
use thiserror::Error;

/// Enum que mapeia todos os tipos de erro que o `crew` pode encontrar.
#[derive(Debug, Error)]
pub enum CrewError {
    // ============================================================================
    // Docker-related Errors
    // ============================================================================
    /// Docker daemon não está rodando ou não é acessível via socket.
    #[error("Docker daemon is unreachable. Is Docker running?")]
    DockerDaemonUnreachable,

    /// Usuário não tem permissões para acessar o socket do Docker.
    #[error(
        "Permission denied: unable to connect to Docker daemon. Check docker permissions or group membership"
    )]
    DockerPermissionDenied,

    /// Erro genérico ao comunicar com Docker via API.
    #[error("Docker API error: {0}")]
    DockerApiError(String),

    // ============================================================================
    // Git-related Errors
    // ============================================================================
    /// Diretório atual não é um repositório Git inicializado (warning, não fatal).
    #[error("Git repository not initialized in the current directory")]
    GitRepositoryNotInitialized,

    /// Erro ao executar comando git via shell.
    #[error("Git command failed: {0}")]
    GitCommandFailed(String),

    // ============================================================================
    // Docker Compose-related Errors
    // ============================================================================
    /// `docker-compose.yml` ou `docker-compose.yaml` não encontrado na raiz.
    #[error("docker-compose.yml not found in current directory")]
    ComposeFileNotFound,

    /// Erro ao parsear o YAML do `docker-compose.yml`.
    #[error("failed to parse docker-compose.yml: {0}")]
    ComposeParseError(String),

    // ============================================================================
    // UI-related Errors
    // ============================================================================
    /// Erro ao renderizar menu interativo (inquire).
    #[error("UI error: {0}")]
    UiError(String),

    // ============================================================================
    // IO Errors (wrapped)
    // ============================================================================
    /// Erro genérico de IO (ler arquivo, escrever, etc.).
    /// Usa `#[from]` para conversão automática de `io::Error`.
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

/// Type alias para `Result<T, CrewError>`.
/// Uso: `pub fn my_function() -> CrewResult<String> { ... }`
pub type CrewResult<T> = Result<T, CrewError>;

// ============================================================================
// Helper Methods para converter erros específicos de bibliotecas
// ============================================================================
impl CrewError {
    /// Cria um `DockerApiError` a partir de uma string genérica.
    /// Útil ao capturar erros do `bollard`.
    pub fn docker_api(msg: impl Into<String>) -> Self {
        CrewError::DockerApiError(msg.into())
    }

    /// Cria um `GitCommandFailed` a partir de uma string.
    /// Útil ao capturar stderr de processos do git.
    pub fn git_command(msg: impl Into<String>) -> Self {
        CrewError::GitCommandFailed(msg.into())
    }

    /// Cria um `ComposeParseError` a partir de uma string.
    /// Útil ao capturar erros do `serde_yaml`.
    pub fn compose_parse(msg: impl Into<String>) -> Self {
        CrewError::ComposeParseError(msg.into())
    }

    /// Cria um `UiError` a partir de uma string.
    /// Útil ao capturar erros do `inquire`.
    pub fn ui(msg: impl Into<String>) -> Self {
        CrewError::UiError(msg.into())
    }

    /// Verifica se este erro é "não-fatal" (deve renderizar UI parcial).
    /// Erros como Git/Docker offline retornam `true`. IO errors retornam `false`.
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            CrewError::DockerDaemonUnreachable
                | CrewError::DockerPermissionDenied
                | CrewError::GitRepositoryNotInitialized
                | CrewError::ComposeFileNotFound
        )
    }

    /// Retorna uma mensagem breve para renderizar como aviso na UI.
    /// Útil para tabelas parciais no `crew status`.
    pub fn ui_warning(&self) -> String {
        match self {
            CrewError::DockerDaemonUnreachable => "⚠️  Docker Daemon Offline".to_string(),
            CrewError::DockerPermissionDenied => "⚠️  Docker Permission Denied".to_string(),
            CrewError::GitRepositoryNotInitialized => "ℹ️  Not a Git Repository".to_string(),
            CrewError::ComposeFileNotFound => "ℹ️  docker-compose.yml Not Found".to_string(),
            _ => format!("⚠️  Error: {}", self),
        }
    }
}
