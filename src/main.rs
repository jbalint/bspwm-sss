//#![allow(unused_imports)]
//#![allow(unused_variables)]

extern crate libc;
extern crate ctrlc;
extern crate time;
extern crate reqwest;

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{ChildStdout, Command, Stdio};
use libc::{kill, SIGTERM};
use reqwest::{RequestBuilder};
use reqwest::header::{Authorization, Basic, ContentType};

const SPARQL_UPDATE_ENDPOINT: &str = "https://localhost/stardog/test/update";

// # (c.f. #4442)
const SPARQL_INSERT_FOCUS: &str =
    "
prefix bspwm-sss: <http://github.com/jbalint/bspwm-sss#>

insert {
  [] a bspwm-sss:FocusEvent ;
    bspwm-sss:monitor ?monitor ;
    bspwm-sss:desktop ?desktop ;
    bspwm-sss:node ?node ;
    bspwm-sss:time ?time
}
where {
  bind(iri(concat(str(bspwm-sss:), ?monitor_id)) as ?monitor)
  bind(iri(concat(str(bspwm-sss:), ?desktop_id)) as ?desktop)
  bind(iri(concat(str(bspwm-sss:), ?node_id)) as ?node)
  bind(?time_ms as ?time)
}
";

fn bspc<'a>() -> Result<BufReader<ChildStdout>, String> {

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
    }).expect("failed");

    Ok(BufReader::new(child.stdout.unwrap()))
}

fn sss_main() -> Result<(), Box<Error>> {

    let mut params = HashMap::new();
    params.insert("query", SPARQL_INSERT_FOCUS.to_string());
    let rclient = reqwest::Client::new();
    let mut req: RequestBuilder = rclient.post(SPARQL_UPDATE_ENDPOINT);
    req.header(ContentType::form_url_encoded())
        .header(Authorization(Basic {
            username: "admin".to_string(),
            password: Some("admin".to_string()),
        }));

    for line in bspc()?.lines() {
        let t = time::get_time();

        params.insert("$monitor_id", "\"1\"".to_string());
        params.insert("$time_ms", format!("\"{}\"", t.sec * 1000));

        req.form(&params).send()?;

        println!("Read a line from bspc: >> {} <<", line.unwrap());
    }

    Ok(())
}

fn main() {
    match sss_main() {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
