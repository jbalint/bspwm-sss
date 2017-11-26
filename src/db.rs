extern crate time;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use reqwest::{Client, RequestBuilder};
use reqwest::header::{Authorization, Basic, ContentType};

use event::*;

const SPARQL_UPDATE_ENDPOINT: &str = "https://localhost/stardog/test/update";

/// SPARQL update for a focus event
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

#[derive(Debug, Clone)]
pub struct DbError;

impl Display for DbError {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Db error")
    }
}

impl Error for DbError {

    fn description(&self) -> &str {
        "db error"
    }

    fn cause(&self) -> Option<&Error> {
        // TODO : put something here
        None
    }
}

pub struct Db {
    client: Client,
}

impl Db {
    pub fn new() -> Db {
        Db { client: Client::new() }
    }

    fn new_req(&self) -> RequestBuilder {
        let mut req: RequestBuilder = self.client.post(SPARQL_UPDATE_ENDPOINT);
        req.header(ContentType::form_url_encoded())
            .header(Authorization(Basic {
                username: "admin".to_string(),
                password: Some("admin".to_string()),
            }));
        req
    }

    pub fn insert(&self, e: &NodeEvent) -> Result<(), DbError> {

        let mut req = self.new_req();

        // TODO : make sure this isn't copying
        let q = SPARQL_INSERT_FOCUS.to_string();
        let t = time::get_time();
        let time_str = format!("\"{}\"", t.sec * 1000);

        let mut params = HashMap::new();
        params.insert("query", &q);
        params.insert("$time_ms", &time_str);
        params.insert("$monitor_id", &e.monitor_id);
        params.insert("$desktop_id", &e.desktop_id);
        params.insert("$node_id", &e.node_id);

        match req.form(&params).send() {
            Ok(_) => Ok(()),
            Err(_) => Err(DbError {}),
        }
    }
}
