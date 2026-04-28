use crate::{
  Result,
  event::Event,
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

impl Default for Dispatch {
  fn default() -> Self {
    let (sender, receiver) = mpsc::unbounded_channel();

    let crossterm_events_handler =
      CrosstermEventsHandlerTask::new(sender.clone());
    let ticker = TickTask::new(sender.clone());

    // take out of default, move ot init or something
    tokio::spawn(async move { ticker.run().await });
    tokio::spawn(async move { crossterm_events_handler.run().await });
    Self { receiver, sender }
  }
}
