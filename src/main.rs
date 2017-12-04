
extern crate libc;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate x11;

use std::str::FromStr;
use std::thread;

mod bspc;
mod db;
mod event;
mod win_track;

use db::Db;
use event::NodeEvent;

mod errors {
    #![allow(unused_doc_comment)]
    error_chain!{}
}

use errors::*;

fn run() -> Result<()> {

    let db = Db::new();

    thread::spawn(move || {
        bspc::bspc_listen(|l| match NodeEvent::from_str(&l).and_then(|e| db.insert(&e)) {
            Ok(_) => (),
            Err(e) => log_error(&e),
        });
    });

    // nighty
    loop { thread::park(); }

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
