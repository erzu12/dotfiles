use std::process::Command;
use std::thread;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use std::{future::pending, error::Error};

use hyprland::data::{Monitors, Workspace, Workspaces};
use hyprland::event_listener::EventListener;
use hyprland::shared::{HyprData, HyprDataActive, HyprDataVec};
use zbus::{ConnectionBuilder, dbus_interface};

use crate::domain::DomainManager;
use crate::workspace::*;

//#[derve(sync
enum HyprEvent {
    WorkspaceDestroyed,
    Update,
    UpdateEwwOnly,
    CreateDomain(String),
    SwitchDomainByName(String),
    AddWorkspaceToDomain(i32),
    SwitchWorkspace(String),
    SendToWorkspace(i32),
    SendToDomain(String),
    GetDomainNames,
}

struct Server {
    tx: UnboundedSender<HyprEvent>,
    name_rx: UnboundedReceiver<Vec<String>>,
}

#[dbus_interface(name = "org.hypr.Server1")]
impl Server {
    fn create_domain(&mut self, name: String) {
        self.tx.send(HyprEvent::CreateDomain(name)).unwrap();
    }
    fn switch_domain_by_name(&mut self, name: String) {
        self.tx.send(HyprEvent::SwitchDomainByName(name)).unwrap();
    }
    fn add_workspace_to_domain(&mut self, workspace: i32) {
        self.tx.send(HyprEvent::AddWorkspaceToDomain(workspace)).unwrap();
    }
    fn switch_worckspace(&self, workspace: String) {
        self.tx.send(HyprEvent::SwitchWorkspace(workspace)).unwrap();
    }
    fn send_to_workspace(&self, workspace: i32) {
        self.tx.send(HyprEvent::SendToWorkspace(workspace)).unwrap();
    }
    fn send_to_domain(&self, name: String) {
        self.tx.send(HyprEvent::SendToDomain(name)).unwrap();
    }
    async fn get_domain_names(&mut self) -> Vec<String> {
        self.tx.send(HyprEvent::GetDomainNames).unwrap();
        self.name_rx.recv().await.unwrap()
    }
    fn update_eww(&self) {
        self.tx.send(HyprEvent::UpdateEwwOnly).unwrap();
    }
}

// Although we use `async-std` here, you can use any async runtime of choice.
pub async fn start_server() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let (name_tx, name_rx) = tokio::sync::mpsc::unbounded_channel();
    let server = Server { tx: tx.clone() , name_rx};

    println!("starting Server...");
    println!("init Dbus...");
    let conn = ConnectionBuilder::session()?
        .name("org.hypr.Server")?
        .serve_at("/org/hypr/Server", server)?
        .build();
    println!("Server awiating...");
    let _conn = conn.await?;

    println!("starting event listener...");
    thread::spawn(move|| {
        event_listener_start(tx).unwrap();
    });

    let mut dm = DomainManager::new();

    println!("Server is running...");

    while let Some(event) = rx.recv().await {
        match event {
            HyprEvent::WorkspaceDestroyed => {
                dm.remove_workspace_from_domain();
                update_workspace_names().unwrap();
                update_eww(&dm);
            },
            HyprEvent::Update => {
                update_workspace_names().unwrap();
                update_eww(&dm);
            },
            HyprEvent::UpdateEwwOnly => {
                update_eww(&dm);
            },
            HyprEvent::CreateDomain(name) => {
                dm.create_domain(name);
                update_workspace_names().unwrap();
                update_eww(&dm);
            },
            HyprEvent::SwitchDomainByName(name) => {
                dm.switch_domain_by_name(name);
                update_workspace_names().unwrap();
                update_eww(&dm);
            },
            HyprEvent::AddWorkspaceToDomain(workspace) => {
                dm.add_workspace_to_domain(workspace);
                update_workspace_names().unwrap();
                update_eww(&dm);
            },
            HyprEvent::SwitchWorkspace(workspace) => {
                switch_worckspace(workspace, &mut dm).unwrap();
                update_workspace_names().unwrap();
                update_eww(&dm);
            },
            HyprEvent::SendToWorkspace(workspace) => {
                send_to_workspace(workspace, &mut dm).unwrap();
                update_workspace_names().unwrap();
                update_eww(&dm);
            },
            HyprEvent::SendToDomain(name) => {
                send_to_domain(name.clone(), &mut dm).unwrap();
                dm.switch_domain_by_name(name);
                update_workspace_names().unwrap();
                update_eww(&dm);
            },
            HyprEvent::GetDomainNames => {
                name_tx.send(dm.get_domain_names()).unwrap();
            },
        }
    }

    // Do other things or go to wait forever
    println!("Server is running...");
    pending::<()>().await;

    Ok(())
}

fn event_listener_start(tx: UnboundedSender<HyprEvent>) -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    let tx_clone = tx.clone();
    event_listener.add_workspace_destroy_handler(move |_| {
        tx_clone.send(HyprEvent::WorkspaceDestroyed).unwrap();
    });

    let tx_clone = tx.clone();
    event_listener.add_workspace_moved_handler(move |_| {
        tx_clone.send(HyprEvent::Update).unwrap();
    });

    let tx_clone = tx.clone();
    event_listener.add_workspace_change_handler(move |_| {
        tx_clone.send(HyprEvent::Update).unwrap();
    });

    let tx_clone = tx.clone();
    event_listener.add_active_monitor_change_handler(move |_| {
        tx_clone.send(HyprEvent::UpdateEwwOnly).unwrap();
    });

    let tx_clone = tx.clone();
    event_listener.add_window_open_handler(move |_| {
        tx_clone.send(HyprEvent::Update).unwrap();
    });

    let tx_clone = tx.clone();
    event_listener.add_window_close_handler(move |_| {
        tx_clone.send(HyprEvent::Update).unwrap();
    });

    let tx_clone = tx.clone();
    event_listener.add_window_moved_handler(move |_| {
        tx_clone.send(HyprEvent::Update).unwrap();
    });

    let tx_clone = tx.clone();
    event_listener.add_window_title_change_handler(move |_| {
        tx_clone.send(HyprEvent::Update).unwrap();
    });


    event_listener.start_listener()
}

fn update_eww(dm: &DomainManager) {
    let domain_workspace = &dm.domains[dm.current_domain].workspaces;
    for mon in Monitors::get().unwrap() {
        let active_workspace = Workspace::get_active().unwrap();

        let workspaces = Workspaces::get().unwrap().to_vec();
        let workspace_literal = workspaces.iter()
            .filter(|w| domain_workspace.contains(&w.id))
            .filter(|e| e.monitor == mon.name)
            .map(|e| format!("(workspace :name \\\"{}\\\" :active {} :id {})", e.name, e.id == active_workspace.id, e.id))
            .collect::<Vec<String>>()
            .join("");
        let workspace_literal = format!("workspace{}_literal=\"(box :space-evenly false {})\"", mon.id + 1, workspace_literal);

        let home = std::env::var("HOME").unwrap();
        Command::new("bash")
            .arg("-c")
            .arg(format!("eww -c {}/.config/eww/bar update {}", home, workspace_literal))
            .output()
            .expect("failed to execute process");
    }
}

