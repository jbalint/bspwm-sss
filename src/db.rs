use std::collections::HashMap;
use reqwest::{Client, RequestBuilder};
use reqwest::header::{Authorization, Basic, ContentType};

use event::*;

use super::errors::*;

const SPARQL_UPDATE_ENDPOINT: &str = "https://localhost/stardog/test/update";

/// SPARQL update for a focus event
// # (c.f. #4442)
const SPARQL_INSERT_NODE_EVENT: &str =
    "
prefix bspwm-sss: <http://github.com/jbalint/bspwm-sss#>

insert {
  [] a ?event_type ;
    bspwm-sss:monitor ?monitor ;
    bspwm-sss:desktop ?desktop ;
    bspwm-sss:node ?node ;
    bspwm-sss:time ?time
}
where {
  bind(iri(concat(str(bspwm-sss:), ?event_type_str, \"Event\")) as ?event_type)
  bind(iri(concat(str(bspwm-sss:), ?monitor_id)) as ?monitor)
  bind(iri(concat(str(bspwm-sss:), ?desktop_id)) as ?desktop)
  bind(iri(concat(str(bspwm-sss:), ?node_id)) as ?node)
  bind(?time_ms as ?time)
}
";

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

    pub fn insert(&self, e: &NodeEvent) -> Result<()> {

        let mut req = self.new_req();

        let time_str = e.time.to_string();
        let type_str = e.event_type.to_string();

        let tostr = |ref v| format!("\"{}\"", v);

        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("query", SPARQL_INSERT_NODE_EVENT.to_string());
        params.insert("$event_type_str", tostr(&type_str));
        params.insert("$monitor_id", tostr(&e.monitor_id));
        params.insert("$desktop_id", tostr(&e.desktop_id));
        params.insert("$node_id",    tostr(&e.node_id));
        params.insert("$time_ms",    tostr(&time_str));

        debug!("Request with params {:?}", params);

        req.form(&params)
            .send()
            .chain_err(|| "Insert failed")
            .map(|_| ())
    }
}
