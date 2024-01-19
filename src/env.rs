use std::{process::{Command, ExitStatus}, path::Path};

pub fn run_shell_command(command: impl AsRef<str>) -> Result<String, ExitStatus> {
    let command = command.as_ref();
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(output.status)
    }
}

pub fn create_venv() {
    if Path::new("venv").exists() {
        return;
    }
    run_shell_command("python3 -m venv venv").expect("failed to create venv");
}

pub fn install_instaloader() {
    let r = run_shell_command("venv/bin/pip install instaloader --upgrade");
    if r.is_err() && Path::new("venv/bin/instaloader").exists() {
        return;
    }
    r.expect("failed to install instaloader");
}
