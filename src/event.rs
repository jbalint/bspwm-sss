extern crate time;

use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use super::errors::*;

#[derive(Clone, Copy, Debug)]
pub enum NodeEventType {
    NodeAdd,
    NodeFocus,
    NodeRemove,
}

impl FromStr for NodeEventType {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {

        match s {
            "node_add"    => Ok(NodeEventType::NodeAdd),
            "node_focus"  => Ok(NodeEventType::NodeFocus),
            "node_remove" => Ok(NodeEventType::NodeRemove),
            _ => bail!("Unrecognized event type string {}", s),
        }
    }
}

impl Display for NodeEventType {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct NodeEvent {
    pub event_type: NodeEventType,
    pub monitor_id: String,
    pub desktop_id: String,
    pub node_id:    String,
    pub time:       i64,
}

impl FromStr for NodeEvent {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {

        let mut iter = s.split_whitespace();

        let mut iter_next = || { iter.next().expect("Expected element") };

        let t = time::get_time();

        Ok(NodeEvent {
            event_type: NodeEventType::from_str(iter_next())?,
            monitor_id: iter_next().to_string(),
            desktop_id: iter_next().to_string(),
            node_id:    iter_next().to_string(),
            time:       t.sec * 1_000 + t.nsec as i64 / 1_000_000,
        })
    }
}
