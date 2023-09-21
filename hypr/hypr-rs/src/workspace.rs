use hyprland::data::{Clients, Monitor, Workspace, Workspaces, Client};
use hyprland::dispatch::*;
use hyprland::prelude::*;

use crate::domain::DomainManager;

pub fn switch_worckspace(workspace: String, dm: &mut DomainManager) -> hyprland::Result<()> {
    let workspace_id;
    if workspace == "--next" || workspace == "--prev" {
        if workspace == "--next" {
            workspace_id = get_relative_workspace_id(dm, 1);
        } else {
            workspace_id = get_relative_workspace_id(dm, -1);
        }
    } else {
        workspace_id = get_workspace_id(workspace.parse().unwrap_or(1), 0, dm);
    }

    if let Some(workspace_id) = workspace_id {
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

pub fn send_to_domain(domain: String, dm: &mut DomainManager) -> hyprland::Result<()> {
    if let Some(client) = Client::get_active().unwrap() {
        dm.switch_domain_by_name(domain);
        let workspace_id = Workspace::get_active().unwrap().id;

        hyprland::dispatch!(MoveToWorkspace,
            WorkspaceIdentifierWithSpecial::Id(workspace_id), 
            Some(WindowIdentifier::Address(client.address)))?;
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

fn get_relative_workspace_id(dm: &mut DomainManager, offset: i32) -> Option<i32> {
    let domain_workspace = &dm.domains[dm.current_domain].workspaces;
    let mon = Monitor::get_active().unwrap();
    let active_workspace = Workspace::get_active().unwrap();

    let workspaces = Workspaces::get().unwrap().to_vec();
    let workspaces = workspaces.iter()
        .filter(|w| domain_workspace.contains(&w.id))
        .filter(|e| e.monitor == mon.name);

    let current_id = workspaces.clone().position(|e| e.id == active_workspace.id)
        .unwrap_or(0) as i32;

    let workspace_count = workspaces.count() as i32;

    get_workspace_id((current_id + offset + workspace_count) % workspace_count + 1, 0, dm)
}
