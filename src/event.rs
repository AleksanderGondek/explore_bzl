use crossterm::event::Event as CrosstermEvent;

#[derive(Clone, Debug)]
pub enum Event {
  Crossterm(CrosstermEvent),
  Quit,
  Tick,
}
