use zbus::{Connection, Result, dbus_proxy};
use hyprland::{dispatch::*, data::{Client, Clients}, shared::HyprDataActiveOptional};

use wofi::*;

mod domain;
mod server;
mod wofi;
mod workspace;


#[tokio::main]
async fn main() -> hyprland::Result<()> {
    //hyprland::dispatch!(Exec, "kitty")?;
    
    let args: Vec<String> = std::env::args().collect();

    let short_help_str = "usage: hypr-rs [command] [(opt)args]\n\n\
                          for more information run: \n  'hypr-rs --help'\n";
    let help_str = "usage: hypr-rs [command] [(opt)args]\n\n\
                    commands:\n  \
                    test                   test command for development\n  \
                    switch <workspace>     switch to workspace (<workspace> can be the id or --next / --prev)\n  \
                    move <workspace_id>    move focused window to workspace\n  \
                    domain                 open domain menu\n  \
                    domainmove             move focused window to domain with menu\n  \
                    domainswitch <domain>  switch to domain (<domain> can be the name or --next / --prev)\n  \
                    server                 start the server\n\n  \
                    --help                 show this help message\n\n\
                    tipp: you can use the daemonize command to start the server in the background:\n  \
                    'daemonize -o <path-to-logfile> <path-to-hypr-rs> server'\n";

    match args.get(1) {
        Some(arg) => {
            match arg.as_str() {
                "test" => test()?,
                "switch" => switch_worckspace(args.get(2).unwrap()).await,
                "move" => move_to_workspace(args.get(2).unwrap().parse().unwrap()).await,
                "domain" => domain_menu().await,
                "server" => server::start_server().await.unwrap(),
                "domainmove" => move_to_domain().await,
                "domainswitch" => switch_domain(args.get(2).unwrap()).await,
                "--help" => println!("{}", help_str),
                _ => println!("{}", short_help_str),
            }
        },
        None => println!("{}", short_help_str),
    }

    hyprland::Result::Ok(())
}

fn test() -> hyprland::Result<()> {
    if let Some(active_window) = Client::get_active().unwrap() {
        hyprland::dispatch!(MoveToWorkspace, WorkspaceIdentifierWithSpecial::Id(100), Some(WindowIdentifier::Address(active_window.address)))?;
    }
    hyprland::Result::Ok(())
}

#[dbus_proxy(
    interface = "org.hypr.Server1",
    default_service = "org.hypr.Server",
    default_path = "/org/hypr/Server"
)]
trait Server {
    fn create_domain(&self, name: String) -> Result<()>;
    fn switch_domain_by_name(&self, name: String) -> Result<()>;
    fn add_workspace_to_domain(&self, workspace: i32) -> Result<()>;
    fn switch_worckspace(&self, workspace: String) -> Result<()>;
    fn send_to_workspace(&self, workspace: i32) -> Result<()>;
    fn send_to_domain(&self, domain: String) -> Result<()>;
    fn get_domain_names(&self) -> Result<Vec<String>>;
}

async fn domain_menu() {
    let connection = Connection::session().await.unwrap();
    let proxy = ServerProxy::new(&connection).await.unwrap();

    let mut domain_names = proxy.get_domain_names().await.unwrap();
    let current_domain = domain_names.pop().unwrap();
    domain_names.retain(|x| x != &current_domain);
    domain_names.insert(0, "".to_string());
    domain_names.insert(0, "new domain".to_string());

    if let Some(wofi_out) = show_wofi(domain_names, "domains") {
        if wofi_out.contains("new domain") {
            if let Some(name) = wofi_input("domain name") {
                proxy.create_domain(name).await.unwrap();
            }
        } else {
            proxy.switch_domain_by_name(wofi_out).await.unwrap();
        }
    }
}

async fn switch_worckspace(arg: &String) {
    let connection = Connection::session().await.unwrap();
    let proxy = ServerProxy::new(&connection).await.unwrap();

    proxy.switch_worckspace(arg.clone()).await.unwrap();
}

async fn move_to_workspace(workspace: i32) {
    let connection = Connection::session().await.unwrap();
    let proxy = ServerProxy::new(&connection).await.unwrap();

    proxy.send_to_workspace(workspace).await.unwrap();
}

async fn move_to_domain() {
    let connection = Connection::session().await.unwrap();
    let proxy = ServerProxy::new(&connection).await.unwrap();

    let mut domain_names = proxy.get_domain_names().await.unwrap();
    let current_domain = domain_names.pop().unwrap();
    domain_names.retain(|x| x != &current_domain);

    if let Some(wofi_out) = show_wofi(domain_names, "move to domain") {
        proxy.send_to_domain(wofi_out).await.unwrap();
    }

}

async fn switch_domain(arg: &String) {
    let connection = Connection::session().await.unwrap();
    let proxy = ServerProxy::new(&connection).await.unwrap();

    let domain;

    if arg == "--next" || arg == "--prev" {
        let mut domain_names = proxy.get_domain_names().await.unwrap();
        let current_domain = domain_names.pop().unwrap();
        let current_domain_index = domain_names.iter().position(|x| x == &current_domain).unwrap();
        let domain_count = domain_names.len();
        if arg == "--next" {
            let next_index = (current_domain_index + 1) % domain_count;
            domain = domain_names[next_index].clone();
        } else {
            let prev_index = (current_domain_index + domain_count - 1) % domain_count;
            domain = domain_names[prev_index].clone();
        }
    }
    else {
        domain = arg.clone();
    }

    proxy.switch_domain_by_name(domain).await.unwrap();
}
