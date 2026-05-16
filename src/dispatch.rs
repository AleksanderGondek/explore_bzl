use crate::{
  Result,
  event::{BazelCommand, Event},
  task::{BazelTask, CrosstermEventsHandlerTask, TickTask},
};

use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Dispatch {
  receiver: mpsc::UnboundedReceiver<Event>,
  sender: mpsc::UnboundedSender<Event>,
}

impl Dispatch {
  pub async fn next(&mut self) -> Result<Event> {
    self.receiver.recv().await.ok_or(crate::Error::Imaginary)
  }

  pub fn send(&mut self, event: Event) {
    if let Event::BazelRequest(command) = &event {
      let bzl_cmd = BazelTask::new(command.clone(), self.sender.clone());
      tokio::spawn(async move { bzl_cmd.run().await });
      return;
    }

    let _ = self.sender.send(event);
  }
}

impl Dispatch {
  #[must_use]
  pub fn init(self) -> Self {
    let bzl_info = BazelTask::new(BazelCommand::Info, self.sender.clone());
    let bzl_targets = BazelTask::new(
      BazelCommand::Query(crate::event::BazelQuery::Targets),
      self.sender.clone(),
    );
    let crossterm_events_handler =
      CrosstermEventsHandlerTask::new(self.sender.clone());
    let ticker = TickTask::new(self.sender.clone());

    // Ticker worker
    tokio::spawn(async move { ticker.run().await });
    // Keyboard events handler
    tokio::spawn(async move { crossterm_events_handler.run().await });
    // Retrieve bazel info
    tokio::spawn(async move { bzl_info.run().await });
    // Retrieve bazel targets
    tokio::spawn(async move { bzl_targets.run().await });

    self
  }
}

impl Default for Dispatch {
  fn default() -> Self {
    let (sender, receiver) = mpsc::unbounded_channel();
    Self { receiver, sender }
  }
}
