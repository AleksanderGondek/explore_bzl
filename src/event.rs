use std::collections::BTreeMap;

use crossterm::event::Event as CrosstermEvent;

use crate::model::{BazelInfo, BazelTarget};

#[derive(Clone, Debug)]
pub enum BazelQuery {
  Custom(Box<Vec<String>>),
  Packages,
  Target(String),
  Targets,
}

#[derive(Clone, Debug)]
pub enum BazelCommand {
  Aquery,
  Build,
  Cquery,
  Info,
  Query(BazelQuery),
}

#[derive(Clone, Debug)]
pub enum BazelCmdResponse {
  Aquery,
  Build,
  Cquery,
  Info(Box<BazelInfo>),
  // TODO: Custom type: Label?
  Query(Box<BTreeMap<String, BazelTarget>>),
}

#[derive(Clone, Debug)]
pub enum Event {
  BazelRequest(BazelCommand),
  BazelResponse(BazelCmdResponse),
  Crossterm(CrosstermEvent),
  Quit,
  SelectDown,
  SelectUp,
  Tick,
}
