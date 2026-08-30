#![allow(dead_code)]

use crate::error::{CrewError, CrewResult};
use std::process::Command;

/// Obtém o nome da branch atual do repositório Git.
///
/// Executa: `git rev-parse --abbrev-ref HEAD`
///
/// # Errors
/// Retorna `CrewError::GitRepositoryNotInitialized` se não for um repo Git válido.
/// Retorna `CrewError::GitCommandFailed` se o comando falhar por outro motivo.
pub fn get_current_branch() -> CrewResult<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .map_err(|e| CrewError::git_command(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Se stderr contém "not a git repository", é erro de repo não inicializado
        if stderr.contains("not a git repository") {
            return Err(CrewError::GitRepositoryNotInitialized);
        }
        return Err(CrewError::git_command(format!(
            "git rev-parse failed: {}",
            stderr
        )));
    }

    let branch = String::from_utf8(output.stdout)
        .map_err(|e| CrewError::git_command(format!("Invalid UTF-8 in git output: {}", e)))?
        .trim()
        .to_string();

    Ok(branch)
}

pub fn list_branches() -> CrewResult<Vec<String>> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--list")
        .output()
        .map_err(|e| CrewError::git_command(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("not a git repository") {
            return Err(CrewError::GitRepositoryNotInitialized);
        }
        return Err(CrewError::git_command(format!(
            "git branch failed: {}",
            stderr
        )));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| CrewError::git_command(format!("Invalid UTF-8 in git output: {}", e)))?;

    // Parse cada linha: remove espaços, asterisco (*), e filtra vazias
    let branches: Vec<String> = stdout
        .lines()
        .map(|line| line.trim_start_matches('*').trim().to_string())
        .filter(|branch| !branch.is_empty())
        .collect();

    Ok(branches)
}

pub fn get_git_status() -> CrewResult<(String, bool)> {
    let current_branch = get_current_branch()?;

    // Verifica se há mudanças: `git status --porcelain` retorna vazio se tudo está committed
    let status_output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .map_err(|e| CrewError::git_command(format!("Failed to execute git status: {}", e)))?;

    let status_str = String::from_utf8(status_output.stdout)
        .map_err(|e| CrewError::git_command(format!("Invalid UTF-8 in git output: {}", e)))?;

    let is_dirty = !status_str.trim().is_empty();

    Ok((current_branch, is_dirty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_branch_format() {}
}
