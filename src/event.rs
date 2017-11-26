use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseError;

impl Display for ParseError {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Parse failed")
    }
}

impl Error for ParseError {

    fn description(&self) -> &str {
        "parse failed"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NodeEventType {
    NodeFocus,
    NodeManage,
    NodeUnmanage,
}

impl FromStr for NodeEventType {

    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s {
            "node_focus" => Ok(NodeEventType::NodeFocus),
            "node_manage" => Ok(NodeEventType::NodeManage),
            "node_unmanager" => Ok(NodeEventType::NodeUnmanage),
            _ => Err(ParseError {}),
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

    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut iter = s.split_whitespace();

        let mut iter_next = || { iter.next().expect("Expected element") };

        Ok(NodeEvent {
            event_type: NodeEventType::from_str(iter_next())?,
            monitor_id: iter_next().to_string(),
            desktop_id: iter_next().to_string(),
            node_id: iter_next().to_string(),
        })
    }
}
