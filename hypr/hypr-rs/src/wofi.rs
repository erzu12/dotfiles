use std::{process::{Command, Stdio}, io::Write};


pub fn show_wofi(items: Vec<String>, prompt: &str) -> Option<String> {
    let home = std::env::var("HOME").unwrap();
    let mut wofi = Command::new("wofi")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-c")
        .arg(format!("{}/.config/hypr/wofi/config", home))
        .arg("--dmenu")
        .arg("--prompt")
        .arg(prompt)
        .arg("--cache-file")
        .arg("/dev/null")
        .spawn()
        .expect("failed to execute process");

    let in_str = items.join("\n");

    let child_stdin = wofi.stdin.as_mut().unwrap();
    child_stdin.write_all(in_str.as_bytes()).unwrap();
    drop(child_stdin);

    let output = wofi.wait_with_output().unwrap();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stdout = stdout.trim();
        if !stdout.is_empty() {
            return Some(stdout.to_string());
        }
    }
    None
}

pub fn wofi_input(prompt: &str) -> Option<String> {
    let home = std::env::var("HOME").unwrap();

    let mut wofi = Command::new("wofi")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-c")
        .arg(format!("{}/.config/hypr/wofi/config", home))
        .arg("--dmenu")
        .arg("--prompt")
        .arg(prompt)
        .arg("-e")
        .arg("--lines")
        .arg("1")
        .spawn()
        .expect("failed to execute process");

    let child_stdin = wofi.stdin.as_mut().unwrap();
    child_stdin.write_all(b" ").unwrap();
    drop(child_stdin);

    let output = wofi.wait_with_output().unwrap();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stdout = stdout.trim();
        if !stdout.is_empty() {
            return Some(stdout.to_string());
        }
    }
    None
}
