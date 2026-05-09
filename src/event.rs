use std::collections::BTreeMap;

use crossterm::event::Event as CrosstermEvent;

use crate::model::BazelInfo;

// TODO: Remove Box<String>
// TODO: Introduce COWs

#[derive(Clone, Debug)]
pub enum BazelQuery {
  Custom(Box<Vec<String>>),
  Packages,
  Target(Box<String>),
  Targets,
}

#[derive(Clone, Debug)]
pub enum BazelCommand {
  Aquery,
  Build,
  Cquery,
  Info,
  Query(BazelQuery),
  QueryForRepr(Box<String>),
}

#[derive(Clone, Debug)]
pub enum BazelCmdResponse {
  Aquery,
  Build,
  Cquery,
  Info(Box<BazelInfo>),
  // TODO: Custom type: Label?
  Query(Box<BTreeMap<String, crate::bazel_proto::blaze_query::Target>>),
  QueryForRepr(Box<(String, Vec<String>)>),
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
