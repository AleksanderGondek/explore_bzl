use crate::{
  Result,
  dispatch::Dispatch,
  event::{BazelCommand, BazelQuery, Event},
  model::Model,
  ui::Ui,
};

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
      handle_crossterm_events(&mut dispatch, &e);
      continue;
    }

    // TODO: Wrap in to separate function(s)?
    // TODO: Return new state, instead of modifying current one?
    match event {
      Event::BazelResponse(crate::event::BazelCmdResponse::Info(info)) => {
        state.bazel_info = *info;
      }
      // TODO: This needs to be way more specific
      Event::BazelResponse(crate::event::BazelCmdResponse::Query(response)) => {
        for (label, target_details) in response.iter() {
          state
            .targets
            .insert(label.to_owned(), target_details.clone());
        }
        if !state.targets.is_empty() && state.targets_selection.is_none() {
          state.targets_selection = Some(0);
        }
        // state.targets = *response;
      }
      Event::Quit => {
        state.should_quit = true;
      }
      Event::SelectUp => match state.targets_selection {
        Some(0) => {
          state.targets_selection = Some(state.targets.len());
        }
        Some(selection) => state.targets_selection = Some(selection - 1),
        _ if !state.targets.is_empty() => state.targets_selection = Some(0),
        _ => (),
      },
      Event::SelectDown => match state.targets_selection {
        Some(selection) => {
          state.targets_selection = Some((selection + 1) % state.targets.len());
          if let Some(target) = state.selected_target() {
            dispatch.send(Event::BazelRequest(BazelCommand::Query(
              BazelQuery::Target(target.label.clone()),
            )));
          }
        }
        _ if !state.targets.is_empty() => state.targets_selection = Some(0),
        _ => (),
      },
      Event::Tick => {
        // TODO: Do a tick thing if needed
      }
      _ => (),
    }
  }

  Ok(())
}

fn handle_crossterm_events(
  dispatch: &mut Dispatch,
  event: &crossterm::event::Event,
) {
  match event {
    crossterm::event::Event::Key(key_event)
      if key_event.kind == crossterm::event::KeyEventKind::Press =>
    {
      match key_event.code {
        crossterm::event::KeyCode::Char('l') => dispatch.send(
          Event::BazelRequest(BazelCommand::Query(BazelQuery::Targets)),
        ),
        crossterm::event::KeyCode::Char('t') => {
          dispatch.send(Event::BazelRequest(BazelCommand::Info));
        }
        crossterm::event::KeyCode::Char('q') => dispatch.send(Event::Quit),
        crossterm::event::KeyCode::Up => dispatch.send(Event::SelectUp),
        crossterm::event::KeyCode::Down => dispatch.send(Event::SelectDown),
        _ => (),
      }
    }
    _ => (),
  }
}
