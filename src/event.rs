use std::str::FromStr;

use super::errors::*;

#[derive(Clone, Copy, Debug)]
pub enum NodeEventType {
    NodeFocus,
    NodeManage,
    NodeUnmanage,
}

impl FromStr for NodeEventType {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {

        match s {
            "node_focus"    => Ok(NodeEventType::NodeFocus),
            "node_manage"   => Ok(NodeEventType::NodeManage),
            "node_unmanage" => Ok(NodeEventType::NodeUnmanage),
            _ => bail!("Unrecognized event type string {}", s),
        }
    }
}

#[derive(Debug)]
pub struct NodeEvent {
    pub event_type: NodeEventType,
    pub monitor_id: String,
    pub desktop_id: String,
    pub node_id:    String,
}

impl FromStr for NodeEvent {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {

        let mut iter = s.split_whitespace();

        let mut iter_next = || { iter.next().expect("Expected element") };

        Ok(NodeEvent {
            event_type: NodeEventType::from_str(iter_next())?,
            monitor_id: iter_next().to_string(),
            desktop_id: iter_next().to_string(),
            node_id:    iter_next().to_string(),
        })
    }
}
