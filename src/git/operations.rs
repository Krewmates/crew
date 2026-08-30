#![allow(dead_code)]

use crate::error::{CrewError, CrewResult};
use std::process::Command;

/// Cria uma nova branch a partir do HEAD atual.

pub fn create_branch(branch_name: &str) -> CrewResult<()> {
    let output = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .output()
        .map_err(|e| CrewError::git_command(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CrewError::git_command(stderr.to_string()));
    }

    Ok(())
}

/// Troca para uma branch existente.
///
pub fn switch_branch(branch_name: &str) -> CrewResult<()> {
    let output = Command::new("git")
        .arg("checkout")
        .arg(branch_name)
        .output()
        .map_err(|e| CrewError::git_command(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CrewError::git_command(stderr.to_string()));
    }

    Ok(())
}

/// Valida se um nome de branch é válido segundo as regras do Git.
///
/// # Arguments
/// * `name` - Nome da branch a validar
///
/// # Returns
/// `true` se válido, `false` caso contrário
pub fn is_valid_branch_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    if name.starts_with('.') || name.starts_with('-') || name.ends_with('-') {
        return false;
    }

    if name.contains(' ') || name.contains("..") {
        return false;
    }

    // Não pode conter caracteres especiais problemáticos
    !name.contains(|c: char| c.is_control() || c == '~' || c == '^' || c == ':')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_branch_names() {
        assert!(is_valid_branch_name("main"));
        assert!(is_valid_branch_name("feat/login"));
        assert!(is_valid_branch_name("fix/bug-123"));
        assert!(is_valid_branch_name("refactor_utils"));
    }

    #[test]
    fn test_invalid_branch_names() {
        assert!(!is_valid_branch_name(""));
        assert!(!is_valid_branch_name("feat "));
        assert!(!is_valid_branch_name(" main"));
        assert!(!is_valid_branch_name("-invalid"));
        assert!(!is_valid_branch_name("invalid-"));
        assert!(!is_valid_branch_name(".invalid"));
        assert!(!is_valid_branch_name("inv..alid"));
    }
}
