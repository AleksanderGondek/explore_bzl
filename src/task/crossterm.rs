use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::{Result, event::Event};

pub struct CrosstermEventsHandlerTask {
  sender: mpsc::UnboundedSender<Event>,
}

impl CrosstermEventsHandlerTask {
  #[must_use]
  pub fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
    Self { sender }
  }

  pub async fn run(&self) -> Result<()> {
    let mut reader = crossterm::event::EventStream::new();
    loop {
      let crossterm_event = reader.next().fuse();
      tokio::select! {
        () = self.sender.closed() => {
          break;
        },
        Some(Ok(e)) = crossterm_event => {
          // TODO: Handle result of sending
          let _ = self.sender.send(Event::Crossterm(e));
        }
      }
    }

    Ok(())
  }
}
