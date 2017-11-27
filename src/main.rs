
//#![allow(unused_imports)]
//#![allow(unused_variables)]

extern crate libc;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate ctrlc;
extern crate reqwest;
#[macro_use]
extern crate error_chain;

use std::io::{BufRead, BufReader, Lines};
use std::process::{ChildStdout, Command, Stdio};
use std::str::FromStr;
use libc::{kill, SIGTERM};

mod db;
mod event;

use db::Db;
use event::{NodeEvent};

mod errors {
    #![allow(unused_doc_comment)]
    error_chain!{}
}

use errors::*;

// TODO : return something more general than ChildStdout
fn bspc<'a>() -> Result<Lines<BufReader<ChildStdout>>> {

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

fn run() -> Result<()> {

    let db = Db::new();

    bspc()?
        .map(|l| NodeEvent::from_str(&l.unwrap()))
        .filter_map(|res| match res {
            Ok(e) => Some(e),
            Err(err) => {
                log_error(&err);
                None
            },
        })
        .for_each(|e| match db.insert(&e) {
            Ok(_) => (),
            Err(e) => log_error(&e),
        });

    Ok(())
}

fn log_error(e: &Error) -> () {

    error!("error: {}", e);

    for e in e.iter().skip(1) {
        error!("caused by: {}", e);
        // TODO : can we get the backtrace of each error in here?
    }

    if let Some(backtrace) = e.backtrace() {
        warn!("backtrace: {:?}", backtrace);
    }
}

fn main() {

    pretty_env_logger::init().unwrap();

    if let Err(ref e) = run() {
        log_error(e);

        ::std::process::exit(1);
    }

}
