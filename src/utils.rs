use std::process::{Command, Output};

pub fn run_command(command: &str) -> Output {
    Command::new("sh").arg("-c").arg(command).output().unwrap()
}
