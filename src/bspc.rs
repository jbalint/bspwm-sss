//! 

extern crate ctrlc;

use libc::{kill, SIGTERM};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

use super::errors::*;

// https://github.com/baskerville/bspwm/blob/master/src/bspc.c
pub fn bspc_listen<F>(listener: F) -> ()
where
    F: Fn(String) -> (),
{
    let child: Child = Command::new("bspc")
        .arg("subscribe")
        .arg("node_add")
        .arg("node_remove")
        .arg("node_focus")
        .arg("node_activate")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // match child.try_wait().unwrap() {
    //     Some(status) => // TODO : get the stdout (bspc doesn't use stderr)
    // }

    let pid = child.id();

    // TODO : is this necessary?
    ctrlc::set_handler(move || unsafe {
        kill(pid as i32, SIGTERM);
        ::std::process::exit(0);
    })
    .chain_err(|| "failed to install signal handler")
    .unwrap();

    BufReader::new(child.stdout.unwrap())
        .lines()
        .map(|l| l.unwrap())
        .for_each(listener);
}
