extern crate ctrlc;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use libc::{kill, SIGTERM};

use super::errors::*;

// https://github.com/baskerville/bspwm/blob/master/src/bspc.c
pub fn bspc_listen<F>(listener: F) -> ()
    where F: Fn(String) -> ()
{

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
    }).chain_err(|| "failed to install signal handler").unwrap();

    BufReader::new(child.stdout.unwrap())
        .lines()
        .map(|l| l.unwrap())
        .for_each(listener);
}
