//! Módulo de comandos Git.
//! Orquestrador de regra de negócio que coordena:
//! - Leitura de status (git/status.rs)
//! - Operações (git/operations.rs)
//! - UI (ui/git_menu.rs)

use crate::error::CrewResult;
use crate::git::operations::{create_branch, is_valid_branch_name, switch_branch};
use crate::git::status::{get_current_branch, get_git_status, list_branches};
use crate::ui::git_menu::{
    self, GitMenuAction, pause_for_user, prompt_create_branch, prompt_switch_branch,
    show_branches_list, show_current_branch, show_error, show_info, show_success,
};

/// Executa o loop principal do menu Git interativo.
pub async fn run_git_menu() -> CrewResult<()> {
    loop {
        let action = git_menu::render_main_menu()?;

        match action {
            GitMenuAction::ViewCurrentBranch => {
                handle_view_current_branch().await;
            }
            GitMenuAction::ListBranches => {
                handle_list_branches().await;
            }
            GitMenuAction::CreateBranch => {
                handle_create_branch().await;
            }
            GitMenuAction::SwitchBranch => {
                handle_switch_branch().await;
            }
            GitMenuAction::Exit => {
                show_info("Exiting Crew Git.");
                break;
            }
        }

        println!();
    }

    Ok(())
}

async fn handle_view_current_branch() {
    match get_git_status() {
        Ok(git_info) => {
            let (branch, is_dirty) = git_info;
            show_current_branch(&branch, is_dirty);
            pause_for_user();
        }
        Err(e) => {
            show_error(&e.ui_warning());
            pause_for_user();
        }
    }
}

async fn handle_list_branches() {
    match get_current_branch() {
        Ok(current) => match list_branches() {
            Ok(branches) => {
                show_branches_list(&branches, &current);
                pause_for_user();
            }
            Err(e) => {
                show_error(&e.ui_warning());
                pause_for_user();
            }
        },
        Err(e) => {
            show_error(&e.ui_warning());
            pause_for_user();
        }
    }
}

async fn handle_create_branch() {
    let full_branch_name = match prompt_create_branch() {
        Ok(name) => name,
        Err(e) => {
            show_error(&e.to_string());
            pause_for_user();
            return;
        }
    };

    if !is_valid_branch_name(&full_branch_name) {
        show_error(&format!(
            "Invalid branch name: '{}'. Check formatting.",
            full_branch_name
        ));
        pause_for_user();
        return;
    }

    match create_branch(&full_branch_name) {
        Ok(()) => {
            show_success(&format!(
                "Branch '{}' created and switched!",
                full_branch_name
            ));
            pause_for_user();
        }
        Err(e) => {
            show_error(&format!("Failed to create branch: {}", e.to_string()));
            pause_for_user();
        }
    }
}

async fn handle_switch_branch() {
    let branches = match list_branches() {
        Ok(b) => b,
        Err(e) => {
            show_error(&e.ui_warning());
            pause_for_user();
            return;
        }
    };

    let selected_branch = match prompt_switch_branch(branches) {
        Ok(branch) => branch,
        Err(e) => {
            show_error(&e.to_string());
            pause_for_user();
            return;
        }
    };

    match switch_branch(&selected_branch) {
        Ok(()) => {
            show_success(&format!("Switched to branch '{}'", selected_branch));
            pause_for_user();
        }
        Err(e) => {
            show_error(&format!("Failed to switch branch: {}", e.to_string()));
            pause_for_user();
        }
    }
}
