use crate::error::{CrewError, CrewResult};
use inquire::{Select, Text};

/// Prefixos padrão para criação de branches.
const BRANCH_PREFIXES: &[&str] = &["feat", "fix", "docs", "chore", "refactor", "test"];

/// Ações disponíveis no menu principal do Git.
#[derive(Debug, Clone, Copy)]
pub enum GitMenuAction {
    ViewCurrentBranch,
    ListBranches,
    CreateBranch,
    SwitchBranch,
    Exit,
}

impl GitMenuAction {
    /// Label exibido no menu interativo (emoji + descrição).
    fn label(self) -> &'static str {
        match self {
            GitMenuAction::ViewCurrentBranch => "📍 View current branch",
            GitMenuAction::ListBranches => "📋 List all branches",
            GitMenuAction::CreateBranch => "✨ Create new branch",
            GitMenuAction::SwitchBranch => "🔀 Switch branch",
            GitMenuAction::Exit => "❌ Exit",
        }
    }
}

impl TryFrom<&str> for GitMenuAction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "📍 View current branch" => Ok(GitMenuAction::ViewCurrentBranch),
            "📋 List all branches" => Ok(GitMenuAction::ListBranches),
            "✨ Create new branch" => Ok(GitMenuAction::CreateBranch),
            "🔀 Switch branch" => Ok(GitMenuAction::SwitchBranch),
            "❌ Exit" => Ok(GitMenuAction::Exit),
            _ => Err(()),
        }
    }
}

/// Renderiza o menu principal do Git e retorna a ação escolhida.
pub fn render_main_menu() -> CrewResult<GitMenuAction> {
    let options: Vec<&str> = [
        GitMenuAction::ViewCurrentBranch,
        GitMenuAction::ListBranches,
        GitMenuAction::CreateBranch,
        GitMenuAction::SwitchBranch,
        GitMenuAction::Exit,
    ]
    .iter()
    .map(|a| a.label())
    .collect();

    let selection = Select::new("Crew Git Menu", options)
        .prompt()
        .map_err(|e| CrewError::ui(format!("Menu selection failed: {}", e)))?;

    GitMenuAction::try_from(selection)
        .map_err(|_| CrewError::ui("Unknown menu action selected".to_string()))
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
    let prefix = Select::new("Select branch prefix:", BRANCH_PREFIXES.to_vec())
        .prompt()
        .map_err(|e| CrewError::ui(format!("Prefix selection failed: {}", e)))?;

    let placeholder = format!("{}/<name>", prefix);
    let branch_name = Text::new(&format!("Enter branch name (will create: {}):", placeholder))
        .prompt()
        .map_err(|e| CrewError::ui(format!("Branch name input failed: {}", e)))?
        .trim()
        .to_string();

    if branch_name.is_empty() {
        return Err(CrewError::ui("Branch name cannot be empty".to_string()));
    }

    Ok(format!("{}/{}", prefix, branch_name))
}

/// Renderiza um menu para selecionar uma branch da lista disponível.
///
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

    Select::new("Select branch to switch to:", branches)
        .prompt()
        .map_err(|e| CrewError::ui(format!("Branch selection failed: {}", e)))
}

/// Renderiza uma mensagem de sucesso.
pub fn show_success(message: &str) {
    println!("✅ {}", message);
}

/// Renderiza uma mensagem de erro.
pub fn show_error(message: &str) {
    println!("❌ {}", message);
}

/// Renderiza uma mensagem informativa.
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

    #[test]
    fn test_menu_action_roundtrip() {
        let actions = [
            GitMenuAction::ViewCurrentBranch,
            GitMenuAction::ListBranches,
            GitMenuAction::CreateBranch,
            GitMenuAction::SwitchBranch,
            GitMenuAction::Exit,
        ];

        for action in &actions {
            let label = action.label();
            let recovered = GitMenuAction::try_from(label).unwrap();
            assert_eq!(
                std::mem::discriminant(action),
                std::mem::discriminant(&recovered)
            );
        }
    }
}