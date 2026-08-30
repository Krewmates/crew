use crate::error::{CrewError, CrewResult};
use std::process::Command;

/// Executa um comando git genérico e trata erros comuns (repo não inicializado, etc).
///
/// # Arguments
/// * `args` - Argumentos para passar ao `git`
///
/// # Errors
/// Retorna `CrewError::GitRepositoryNotInitialized` se não for um repo Git válido.
/// Retorna `CrewError::GitCommandFailed` para falhas genéricas.
fn run_git(args: &[&str]) -> CrewResult<std::process::Output> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| CrewError::git_command(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("not a git repository") {
            return Err(CrewError::GitRepositoryNotInitialized);
        }
        return Err(CrewError::git_command(format!(
            "git {} failed: {}",
            args.join(" "),
            stderr
        )));
    }

    Ok(output)
}

/// Obtém o nome da branch atual do repositório Git.
///
/// Executa: `git rev-parse --abbrev-ref HEAD`
pub fn get_current_branch() -> CrewResult<String> {
    let output = run_git(&["rev-parse", "--abbrev-ref", "HEAD"])?;

    let branch = String::from_utf8(output.stdout)
        .map_err(|e| CrewError::git_command(format!("Invalid UTF-8 in git output: {}", e)))?
        .trim()
        .to_string();

    Ok(branch)
}

/// Lista todas as branches locais.
///
/// Executa: `git branch --list`
pub fn list_branches() -> CrewResult<Vec<String>> {
    let output = run_git(&["branch", "--list"])?;

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| CrewError::git_command(format!("Invalid UTF-8 in git output: {}", e)))?;

    let branches: Vec<String> = stdout
        .lines()
        .map(|line| line.strip_prefix('*').unwrap_or(line).trim().to_string())
        .filter(|branch| !branch.is_empty())
        .collect();

    Ok(branches)
}

/// Retorna a branch atual e se o working tree está "dirty".
///
/// Combina `get_current_branch` + `git status --porcelain`.
pub fn get_git_status() -> CrewResult<(String, bool)> {
    let current_branch = get_current_branch()?;

    let output = run_git(&["status", "--porcelain"])?;
    let status_str = String::from_utf8(output.stdout)
        .map_err(|e| CrewError::git_command(format!("Invalid UTF-8 in git output: {}", e)))?;

    let is_dirty = !status_str.trim().is_empty();

    Ok((current_branch, is_dirty))
}