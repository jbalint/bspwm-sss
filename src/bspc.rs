extern crate ctrlc;

use std::io::{BufRead, BufReader, Lines};
use std::process::{ChildStdout, Command, Stdio};
use libc::{kill, SIGTERM};

use super::errors::*;

// TODO : return something more general than ChildStdout
pub fn bspc() -> Result<Lines<BufReader<ChildStdout>>> {

    let child = Command::new("bspc")
        .arg("subscribe")
        .arg("node_manage")
        .arg("node_unmanage")
        .arg("node_focus")
        .arg("node_activate")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed");

    let pid = child.id();

    ctrlc::set_handler(move || unsafe {
        kill(pid as i32, SIGTERM);
    }).chain_err(|| "failed to install signal handler")?;

    Ok(BufReader::new(child.stdout.unwrap()).lines())
}
