use std::collections::HashSet;

extern crate fs2;

use hyprland::data::{Workspaces, Monitors};
use hyprland::dispatch::*;
use hyprland::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub workspaces: Vec<i32>,
    pub active_workspaces: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainManager {
    pub domains: Vec<Domain>,
    pub current_domain: usize,
}

impl DomainManager {
    pub fn new() -> DomainManager {
        let workspaces = Workspaces::get().unwrap().to_vec();
        let ids = workspaces.iter().map(|w| w.id).collect::<Vec<i32>>();

        DomainManager {
            domains: vec![Domain {
                name: "default".to_string(),
                workspaces: ids,
                active_workspaces: Self::get_active_workspaces(),
            }],
            current_domain: 0,
        }
    }

    pub fn create_domain(&mut self, name: String) {
        self.domains.push(Domain {
            name,
            workspaces: Vec::new(),
            active_workspaces: Vec::new(),
        });
        self.domains[self.current_domain].active_workspaces = Self::get_active_workspaces();
        self.current_domain = self.domains.len() - 1;
        let max_workspace_id = Workspaces::get().unwrap().to_vec().iter().map(|w| w.id).max().unwrap();
        hyprland::data::Monitors::get().unwrap().to_vec().iter().for_each(|mon| {
            println!("Creating workspace on monitor {}", mon.id);
            hyprland::dispatch!(FocusMonitor, MonitorIdentifier::Id(mon.id as u8)).unwrap();
            hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(max_workspace_id + mon.id as i32 + 1)).unwrap();
            self.add_workspace_to_domain(max_workspace_id + mon.id as i32 + 1);
        });
        hyprland::dispatch!(FocusMonitor, MonitorIdentifier::Id(0)).unwrap();
    }

    pub fn switch_domain_by_name(&mut self, name: String) {
        println!("Switching to domain '{}'", name);
        let domain_index = self.domains.iter().position(|d| d.name == name).unwrap();
        self.switch_domain(domain_index as i32);
    }

    fn switch_domain(&mut self, domain: i32) {
        self.domains[self.current_domain].active_workspaces = Self::get_active_workspaces();
        self.current_domain = domain as usize;
        Monitors::get().unwrap().to_vec().iter().for_each(|m| {
            let active_workspace = self.domains[self.current_domain].active_workspaces[m.id as usize];
            hyprland::dispatch!(FocusMonitor, MonitorIdentifier::Id(m.id as u8)).unwrap();
            hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(active_workspace)).unwrap();
        });
        hyprland::dispatch!(FocusMonitor, MonitorIdentifier::Id(0)).unwrap();
    }

    pub fn add_workspace_to_domain(&mut self, workspace: i32) {
        self.domains[self.current_domain].workspaces.push(workspace);
    }

    pub fn remove_workspace_from_domain(&mut self) {
        let workspace_ids = Workspaces::get().unwrap().to_vec().iter().map(|w| w.id).collect::<HashSet<i32>>();
        self.domains[self.current_domain].workspaces.retain(|&w| workspace_ids.contains(&w));
    }
    
    fn get_active_workspaces() -> Vec<i32> {
        Monitors::get().unwrap().to_vec().iter()
            .map(|m| m.active_workspace.id)
            .collect::<Vec<i32>>()
    }

    pub fn get_domain_names(&self) -> Vec<String> {
        self.domains.iter().map(|d| d.name.clone()).collect::<Vec<String>>()
    }
}

