use crate::{Result, dispatch::Dispatch, event::Event, model::Model, ui::Ui};

use ratatui::DefaultTerminal;

pub async fn run(mut terminal: DefaultTerminal) -> Result<()> {
  let mut dispatch = Dispatch::default();
  let mut state = Model::default();

  while !state.should_quit {
    terminal.try_draw(|frame| {
      frame.render_stateful_widget(Ui::default(), frame.area(), &mut state);
      std::io::Result::Ok(())
    })?;

    // TODO: Wrap into separate function?
    let event = dispatch.next().await?;
    if let Event::Crossterm(e) = event {
      handle_crossterm_events(&mut dispatch, e);
      continue;
    }

    // TODO: Wrap in to separate function(s)?
    // TODO: Return new state, instead of modifying current one?
    match event {
      Event::Quit => {
        state.should_quit = true;
      }
      Event::Tick => (),
      _ => (),
    }
  }

  Ok(())
}

fn handle_crossterm_events(
  dispatch: &mut Dispatch,
  event: crossterm::event::Event,
) {
  match event {
    crossterm::event::Event::Key(key_event)
      if key_event.kind == crossterm::event::KeyEventKind::Press =>
    {
      match key_event.code {
        crossterm::event::KeyCode::Char('q') => dispatch.send(Event::Quit),
        _ => (),
      }
    }
    _ => (),
  }
}
