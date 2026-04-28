use crossterm::event::Event as CrosstermEvent;

use crate::model::BazelInfo;

#[derive(Clone, Debug)]
pub enum BazelCommand {
  Aquery,
  Build,
  Cquery,
  Info,
  Query,
}

#[derive(Clone, Debug)]
pub enum BazelCmdResponse {
  Aquery,
  Build,
  Cquery,
  Info(Box<BazelInfo>),
  Query,
}

#[derive(Clone, Debug)]
pub enum Event {
  BazelRequest(BazelCommand),
  BazelResponse(BazelCmdResponse),
  Crossterm(CrosstermEvent),
  Quit,
  Tick,
}
