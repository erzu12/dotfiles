use std::process::Command;

use hyprland::data::{Client, Clients, Monitor, Workspace, Workspaces, Monitors};
use hyprland::dispatch::*;
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::keyword::*;
use hyprland::prelude::*;
use hyprland::shared::WorkspaceType;
use hyprland::config::binds::*;


fn main() -> hyprland::Result<()> {
    //hyprland::dispatch!(Exec, "kitty")?;
    
    let args: Vec<String> = std::env::args().collect();

    match args.get(1) {
        Some(arg) => {
            match arg.as_str() {
                "test" => update_workspace_names()?,
                "switch" => switch_worckspace(args.get(2).unwrap().parse().unwrap())?,
                "move" => send_to_workspace(args.get(2).unwrap().parse().unwrap())?,
                "event" => event_listener_start()?,
                _ => println!("No such command"),
            }
        },
        None => println!("usage: hypr-rs <command> <options>"),
    }

    hyprland::Result::Ok(())
}

fn test() -> hyprland::Result<()> {
    hyprland::dispatch!(Exec, "kitty")?;
    hyprland::Result::Ok(())
}

fn switch_worckspace(workspace: i32) -> hyprland::Result<()> {

    if let Some(workspace_id) = get_workspace_id(workspace, 0) {
        println!("workspace_id: {}", workspace_id);
        hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(workspace_id))?;
    }
    hyprland::Result::Ok(())
}

fn send_to_workspace(workspace: i32) -> hyprland::Result<()> {
    if let Some(workspace_id) = get_workspace_id(workspace, 1) {
        hyprland::dispatch!(MoveToWorkspace, WorkspaceIdentifierWithSpecial::Id(workspace_id), None)?;
    }
    hyprland::Result::Ok(())
}

fn event_listener_start() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    //event_listener.add_workspace_added_handler(|_, id| {
        //println!("workspace was added: {id:#?}");
        //update_eww()
    //});

    event_listener.add_workspace_moved_handler(|_, _| {
        update_workspace_names().unwrap();
        update_eww()
    });

    event_listener.add_workspace_change_handler(|_, _| {
        update_workspace_names().unwrap();
        update_eww()
    });

    //event_listener.add_workspace_destroy_handler(|_, id| {
        //println!("workspace was destroyed: {id:#?}");
        //update_eww()
    //});
    
    event_listener.add_active_monitor_change_handler(|_, _| {
        update_eww()
    });

    event_listener.add_window_open_handler(|_, _| {
        update_workspace_names().unwrap();
        update_eww()
    });

    event_listener.add_window_close_handler(|_, _| {
        update_workspace_names().unwrap();
        update_eww()
    });

    event_listener.add_window_moved_handler(|_, _| {
        update_workspace_names().unwrap();
        update_eww()
    });

    event_listener.add_window_title_change_handler(|_, _| {
        update_workspace_names().unwrap();
        update_eww()
    });


    event_listener.start_listener()
}

fn update_eww() {
    for mon in Monitors::get().unwrap() {
        let active_workspace = Workspace::get_active().unwrap();

        let workspaces = Workspaces::get().unwrap().to_vec();
        let workspace_literal = workspaces.iter()
            .filter(|e| e.monitor == mon.name)
            .map(|e| format!("(workspace :name \\\"{}\\\" :active {} :id {})", e.name, e.id == active_workspace.id, e.id))
            .collect::<Vec<String>>()
            .join("");
        let workspace_literal = format!("workspace{}_literal=\"(box :space-evenly false {})\"", mon.id + 1, workspace_literal);

        let home = std::env::var("HOME").unwrap();
        Command::new("bash")
            .arg("-c")
            .arg(format!("eww -c {}/.config/eww/bar update {}", home, workspace_literal))
            .spawn()
            .expect("failed to execute process");
    }

}

fn update_workspace_names() -> hyprland::Result<()>{

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

fn get_workspace_id(workspace: i32, min_win: u16) -> Option<i32> {
    let mon = Monitor::get_active().unwrap();

    let active_workspace = Workspace::get_active().unwrap();

    let workspaces = Workspaces::get().unwrap().to_vec();
    let workspace_id = workspaces.iter()
        .filter(|e| e.monitor == mon.name)
        .map(|e| e.id)
        .nth(workspace as usize - 1)
        .unwrap_or_else(|| {
            if active_workspace.windows == min_win {
                return active_workspace.id;
            }
            workspaces.iter()
                .map(|e| e.id)
                .max()
                .unwrap() + 1
        });


    if active_workspace.id == workspace_id {
        return None;
    }

    Some(workspace_id)
}
