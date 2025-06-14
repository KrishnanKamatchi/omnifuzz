
use std::process::Command;

pub fn run_hook(script: &str, arg: &str) {
    let _ = Command::new(script).arg(arg).spawn();
}
