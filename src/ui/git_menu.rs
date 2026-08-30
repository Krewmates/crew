use crate::error::{CrewError, CrewResult};
use inquire::{Select, Text};

/// Prefixos padrão para criação de branches.
const BRANCH_PREFIXES: &[&str] = &["feat", "fix", "docs", "chore", "refactor", "test"];

/// Menu principal do Git. Retorna a ação escolhida pelo usuário.
#[derive(Debug, Clone, Copy)]
pub enum GitMenuAction {
    ViewCurrentBranch,
    ListBranches,
    CreateBranch,
    SwitchBranch,
    Exit,
}

/// Renderiza o menu principal do Git e retorna a ação escolhida.

pub fn render_main_menu() -> CrewResult<GitMenuAction> {
    let options = vec![
        "📍 View current branch",
        "📋 List all branches",
        "✨ Create new branch",
        "🔀 Switch branch",
        "❌ Exit",
    ];

    let selection = Select::new("Crew Git Menu", options)
        .prompt()
        .map_err(|e| CrewError::ui(format!("Menu selection failed: {}", e)))?;

    let action = match selection {
        "📍 View current branch" => GitMenuAction::ViewCurrentBranch,
        "📋 List all branches" => GitMenuAction::ListBranches,
        "✨ Create new branch" => GitMenuAction::CreateBranch,
        "🔀 Switch branch" => GitMenuAction::SwitchBranch,
        "❌ Exit" => GitMenuAction::Exit,
        _ => GitMenuAction::Exit, // Fallback (nunca deve acontecer)
    };

    Ok(action)
}

/// Renderiza o fluxo de criação de branch com seleção de prefixo + nome.
///
/// # Returns
/// Nome completo da branch (ex: "feat/novo-dashboard")
///
/// # Errors
/// Retorna `CrewError::UiError` se:
/// - Usuário cancelar (Ctrl+C)
/// - Nome vazio ou inválido
pub fn prompt_create_branch() -> CrewResult<String> {
    // 1. Selecionar prefixo
    let prefix = Select::new("Select branch prefix:", BRANCH_PREFIXES.to_vec())
        .prompt()
        .map_err(|e| CrewError::ui(format!("Prefix selection failed: {}", e)))?;

    // 2. Digitar nome da branch
    let branch_name = Text::new(&format!(
        "Enter branch name (will create: {}/{}):",
        prefix, "<name>"
    ))
    .prompt()
    .map_err(|e| CrewError::ui(format!("Branch name input failed: {}", e)))?
    .trim()
    .to_string();

    if branch_name.is_empty() {
        return Err(CrewError::ui("Branch name cannot be empty".to_string()));
    }

    // 3. Retornar nome completo
    Ok(format!("{}/{}", prefix, branch_name))
}

/// Renderiza um menu para selecionar uma branch da lista disponível.
/// # Errors
/// Retorna `CrewError::UiError` se:
/// - Lista de branches vazia
/// - Usuário cancelar (Ctrl+C)
pub fn prompt_switch_branch(branches: Vec<String>) -> CrewResult<String> {
    if branches.is_empty() {
        return Err(CrewError::ui(
            "No branches available to switch to".to_string(),
        ));
    }

    let selected = Select::new("Select branch to switch to:", branches)
        .prompt()
        .map_err(|e| CrewError::ui(format!("Branch selection failed: {}", e)))?;

    Ok(selected)
}

/// Renderiza uma mensagem de sucesso (apenas print bonito).
pub fn show_success(message: &str) {
    println!("✅ {}", message);
}

/// Renderiza uma mensagem de erro (apenas print bonito).
pub fn show_error(message: &str) {
    println!("❌ {}", message);
}

/// Renderiza uma mensagem de informação (apenas print bonito).
pub fn show_info(message: &str) {
    println!("ℹ️  {}", message);
}

/// Renderiza a branch atual de forma legível.
pub fn show_current_branch(branch: &str, is_dirty: bool) {
    let status = if is_dirty {
        "🔴 (dirty)"
    } else {
        "🟢 (clean)"
    };
    println!("📍 Current branch: {} {}", branch, status);
}

/// Renderiza a lista de branches de forma legível.
pub fn show_branches_list(branches: &[String], current: &str) {
    println!("\n📋 Available branches:\n");
    for branch in branches {
        let marker = if branch == current { "→" } else { " " };
        println!("  {} {}", marker, branch);
    }
    println!();
}

/// Aguarda que o usuário pressione ENTER para continuar.
pub fn pause_for_user() {
    let _ = Text::new("Press ENTER to continue...").prompt().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_prefixes_defined() {
        assert!(!BRANCH_PREFIXES.is_empty());
        assert!(BRANCH_PREFIXES.contains(&"feat"));
        assert!(BRANCH_PREFIXES.contains(&"fix"));
        assert!(BRANCH_PREFIXES.contains(&"docs"));
    }
}
