use std::time::Duration;

use crate::{Result, event::Event};

use futures::{FutureExt, StreamExt};
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
    let _ = self.sender.send(event);
  }
}

impl Default for Dispatch {
  fn default() -> Self {
    let (sender, receiver) = mpsc::unbounded_channel();
    let terminal_worker = TerminalTask::new(sender.clone());
    tokio::spawn(async { terminal_worker.start().await });
    Self { receiver, sender }
  }
}

struct TerminalTask {
  sender: mpsc::UnboundedSender<Event>,
}

impl TerminalTask {
  fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
    Self { sender }
  }

  async fn start(self) -> () {
    const TICK_FPS: f64 = 30.0;
    let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
    let mut reader = crossterm::event::EventStream::new();
    let mut tick = tokio::time::interval(tick_rate);

    loop {
      let tick_delay = tick.tick();
      let crossterm_event = reader.next().fuse();
      tokio::select! {
        () = self.sender.closed() => {
          break;
        },
        _ = tick_delay => {
          self.send(Event::Tick);
        }
        Some(Ok(e)) = crossterm_event => {
          self.send(Event::Crossterm(e));
        }
      }
    }
  }

  fn send(&self, event: Event) {
    let _ = self.sender.send(event);
  }
}
