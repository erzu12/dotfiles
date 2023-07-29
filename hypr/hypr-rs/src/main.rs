use zbus::{Connection, Result, dbus_proxy};
use hyprland::dispatch::*;

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
                    switch <workspace_id>  switch to workspace\n  \
                    move <workspace_id>    move focused window to workspace\n  \
                    domain                 open domain menu\n  \
                    server                 start the server\n\n  \
                    --help                 show this help message\n\n\
                    tipp: you can use the daemonize command to start the server in the background:\n  \
                    'daemonize -o <path-to-logfile> <path-to-hypr-rs> server'\n";

    match args.get(1) {
        Some(arg) => {
            match arg.as_str() {
                "test" => test()?,
                "switch" => switch_worckspace(args.get(2).unwrap().parse().unwrap()).await,
                "move" => move_to_workspace(args.get(2).unwrap().parse().unwrap()).await,
                "domain" => domain_menu().await,
                "server" => server::start_server().await.unwrap(),
                "--help" => println!("{}", help_str),
                _ => println!("{}", short_help_str),
            }
        },
        None => println!("{}", short_help_str),
    }

    hyprland::Result::Ok(())
}

fn test() -> hyprland::Result<()> {
    hyprland::dispatch!(Exec, "kitty")?;
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
    fn switch_worckspace(&self, workspace: i32) -> Result<()>;
    fn send_to_workspace(&self, workspace: i32) -> Result<()>;
    fn get_domain_names(&self) -> Result<Vec<String>>;
}

async fn domain_menu() {
    let connection = Connection::session().await.unwrap();
    let proxy = ServerProxy::new(&connection).await.unwrap();

    let mut domain_names = proxy.get_domain_names().await.unwrap();
    domain_names.push("".to_string());
    domain_names.push("new domain".to_string());

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

async fn switch_worckspace(workspace: i32) {
    let connection = Connection::session().await.unwrap();
    let proxy = ServerProxy::new(&connection).await.unwrap();

    proxy.switch_worckspace(workspace).await.unwrap();
}

async fn move_to_workspace(workspace: i32) {
    let connection = Connection::session().await.unwrap();
    let proxy = ServerProxy::new(&connection).await.unwrap();

    proxy.send_to_workspace(workspace).await.unwrap();
}
