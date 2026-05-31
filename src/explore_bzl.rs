use crate::{
  Result,
  dispatch::Dispatch,
  event::{BazelCommand, Event},
  model::{Model, Pane},
  ui::Ui,
};

use ratatui::DefaultTerminal;

pub async fn run(mut terminal: DefaultTerminal) -> Result<()> {
  let mut dispatch = Dispatch::default().init();
  let mut state = Model::default().init();

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
      }
      Event::BazelResponse(crate::event::BazelCmdResponse::Cquery(r)) => {
        for (label, target_details) in r.iter() {
          state
            .targets_cquery
            .insert(label.clone(), target_details.clone());
        }
      }
      Event::BazelResponse(crate::event::BazelCmdResponse::QueryForRepr(r)) => {
        let (target, starlark_repr) = *r;
        state.targets_repr.insert(target, starlark_repr);
      }
      Event::Quit => {
        state.should_quit = true;
      }
      Event::SelectPane(pane) => {
        state.selected_pane = pane;
      }
      Event::SelectUp => {
        target_selection(&mut dispatch, &mut state, &TargetSelection::Up);
      }
      Event::SelectDown => {
        target_selection(&mut dispatch, &mut state, &TargetSelection::Down);
      }
      Event::Tick => {
        // TODO: Do a tick thing if needed
      }
      _ => (),
    }
  }

  Ok(())
}

#[derive(Clone, Debug)]
enum TargetSelection {
  Up,
  Down,
}

fn target_selection(
  dispatch: &mut Dispatch,
  state: &mut Model,
  op: &TargetSelection,
) {
  #[allow(clippy::unnecessary_find_map)]
  let Some(current_index) =
    state
      .targets
      .keys()
      .enumerate()
      .find_map(|(i, target_label)| match &state.selected_target {
        // If I roll select_abel == target lable into match, it won't work
        Some(selected_label) => {
          if *selected_label == *target_label {
            Some(i)
          } else {
            None
          }
        }
        _ => Some(0),
      })
  else {
    return;
  };

  let new_index = match op {
    TargetSelection::Up => {
      if current_index == 0 {
        state.targets.len() - 1
      } else {
        current_index - 1
      }
    }
    TargetSelection::Down => (current_index + 1) % state.targets.len(),
  };

  state.selected_target =
    state
      .targets
      .keys()
      .enumerate()
      .find_map(|(i, target_label)| {
        if i == new_index {
          Some(target_label.clone())
        } else {
          None
        }
      });

  if let Some(selected_target) = &state.selected_target.clone() {
    // TODO: How to approach state invalidation?
    // perhaps be naive and re-take each time..
    dispatch.send(Event::BazelRequest(BazelCommand::QueryForRepr(Box::new(
      selected_target.clone(),
    ))));

    // match state.selected_pane {
    //   Pane::Config => {
    dispatch.send(Event::BazelRequest(BazelCommand::Cquery(
      crate::event::BazelQuery::Target(Box::new(selected_target.clone())),
    )));
    //   }
    //   _ => (),
    // }
  }
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
        crossterm::event::KeyCode::Char('c') => {
          dispatch.send(Event::SelectPane(Pane::Config));
        }
        crossterm::event::KeyCode::Char('r') => {
          dispatch.send(Event::SelectPane(Pane::StarlarkRepr));
        }
        crossterm::event::KeyCode::Char('t') => {
          dispatch.send(Event::SelectPane(Pane::Attributes));
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
