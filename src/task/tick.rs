use std::time::Duration;

use tokio::sync::mpsc;

use crate::{Result, event::Event};

pub struct TickTask {
  sender: mpsc::UnboundedSender<Event>,
}

impl TickTask {
  #[must_use]
  pub fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
    Self { sender }
  }

  pub async fn run(&self) -> Result<()> {
    const TICK_FPS: f64 = 30.0;
    let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
    let mut tick = tokio::time::interval(tick_rate);

    loop {
      let tick_delay = tick.tick();
      tokio::select! {
        () = self.sender.closed() => {
          break;
        },
        _ = tick_delay => {
          // TODO: Handle the result of sending
          let _ = self.sender.send(Event::Tick);
        }
      }
    }

    Ok(())
  }
}
