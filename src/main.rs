
//#![allow(unused_imports)]
//#![allow(unused_variables)]

extern crate libc;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate ctrlc;
extern crate reqwest;

use std::error::Error;
use std::io::{BufRead, BufReader, Lines};
use std::process::{ChildStdout, Command, Stdio};
use std::str::FromStr;
use libc::{kill, SIGTERM};

mod db;
mod event;

use db::Db;
use event::{NodeEvent};

// TODO : return something more general than ChildStdout
fn bspc<'a>() -> Lines<BufReader<ChildStdout>> {

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
    }).expect("failed to install signal handler");

    BufReader::new(child.stdout.unwrap()).lines()
}

fn sss_main() -> Result<(), Box<Error>> {

    let db = Db::new();

    bspc()
        .map(|l| NodeEvent::from_str(&l.unwrap()).unwrap())
        .for_each(|e| { db.insert(&e); });

    Ok(())
}

fn main() {

    pretty_env_logger::init().unwrap();

    match sss_main() {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
