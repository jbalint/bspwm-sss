
extern crate libc;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate reqwest;
#[macro_use]
extern crate error_chain;

use std::str::FromStr;

mod bspc;
mod db;
mod event;

use db::Db;
use event::NodeEvent;

mod errors {
    #![allow(unused_doc_comment)]
    error_chain!{}
}

use errors::*;

fn run() -> Result<()> {

    let db = Db::new();

    bspc::bspc()?
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
