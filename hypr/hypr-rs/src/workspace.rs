use hyprland::data::{Clients, Monitor, Workspace, Workspaces};
use hyprland::dispatch::*;
use hyprland::prelude::*;

use crate::domain::DomainManager;

pub fn switch_worckspace(workspace: i32, dm: &mut DomainManager) -> hyprland::Result<()> {

    if let Some(workspace_id) = get_workspace_id(workspace, 0, dm) {
        println!("workspace_id: {}", workspace_id);
        hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(workspace_id))?;
    }
    hyprland::Result::Ok(())
}

pub fn send_to_workspace(workspace: i32, dm: &mut DomainManager) -> hyprland::Result<()> {
    if let Some(workspace_id) = get_workspace_id(workspace, 1, dm) {
        hyprland::dispatch!(MoveToWorkspace, WorkspaceIdentifierWithSpecial::Id(workspace_id), None)?;
    }
    hyprland::Result::Ok(())
}

pub fn update_workspace_names() -> hyprland::Result<()>{

    let clients = Clients::get().unwrap().to_vec();
    let workspaces = Workspaces::get().unwrap().to_vec();

    for workspace in workspaces {
        let workspace_id = workspace.id;

        let mut workspace_name = clients.iter()
            .filter(|e| e.workspace.id == workspace_id)
            .filter(|e| e.mapped)
            .take(4)
            .map(|e| if e.title.len() < 8 {e.title.clone()} else {e.class.clone()})
            .collect::<Vec<String>>()
            .join(" | ");

        if workspace_name.is_empty() {
            workspace_name = "Empty".to_string();
        }

        hyprland::dispatch!(RenameWorkspace, workspace_id, Some(workspace_name.as_str()))?;
    }

    return hyprland::Result::Ok(())
}

fn get_workspace_id(workspace: i32, min_win: u16, dm: &mut DomainManager) -> Option<i32> {
    let mon = Monitor::get_active().unwrap();

    let active_workspace = Workspace::get_active().unwrap();

    let ids = &dm.domains.get(dm.current_domain).unwrap().workspaces;

    let workspaces = Workspaces::get().unwrap().to_vec();
    let workspace_id = workspaces.iter()
        .filter(|e| e.monitor == mon.name && ids.contains(&e.id))
        .map(|e| e.id)
        .nth(workspace as usize - 1)
        .unwrap_or_else(move || {
            if active_workspace.windows == min_win {
                return active_workspace.id;
            }
            let dm_workspace = dm.domains.iter().map(|e| e.workspaces.clone()).flatten().collect::<Vec<i32>>();
            let new_workspaces = dm_workspace.iter()
                .max()
                .unwrap() + 1;
            dm.add_workspace_to_domain(new_workspaces);
            println!("new workspace: {}", new_workspaces);
            new_workspaces
        });


    if active_workspace.id == workspace_id {
        return None;
    }

    Some(workspace_id)
}
