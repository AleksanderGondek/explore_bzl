use ratatui::{
  buffer::Buffer,
  layout::{Constraint, Layout, Offset, Rect},
  style::Stylize,
  symbols,
  widgets::{
    Block, Borders, List, ListState, Padding, Paragraph, StatefulWidget, Tabs,
    Widget,
  },
};

use crate::model::Model;

// TODO: In future move this into ui.rs and kill this module
#[derive(Debug, Default)]
pub struct Ui {}

impl StatefulWidget for Ui {
  type State = Model;
  // TODO: split render
  #[allow(clippy::too_many_lines)]
  fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
    let main_layout = Layout::default()
      .direction(ratatui::layout::Direction::Vertical)
      .constraints(vec![Constraint::Ratio(15, 100), Constraint::Ratio(85, 100)])
      .split(area);

    // Top Row Layout:
    // * Column 1: Bazel Statues
    // * Column 2: explore_bzl advert
    let top_row_layout = Layout::default()
      .direction(ratatui::layout::Direction::Horizontal)
      .constraints(vec![Constraint::Ratio(85, 100), Constraint::Ratio(15, 100)])
      .split(main_layout[0]);

    let statuses: Vec<ratatui::text::Line<'static>> = vec![
      ratatui::text::Line::from_iter([
        "Bazel binary: ".bold(),
        state
          .bazel_binary
          .clone()
          .map(|pth| pth.clone().to_string_lossy().to_string())
          .unwrap()
          .into(),
      ]),
      ratatui::text::Line::from_iter([
        "Bazel version: ".bold(),
        state
          .bazel_info
          .release
          .clone()
          .unwrap_or("<ELO>".to_string())
          .into(),
      ]), //TODO: This is ugly, needs fixing
      ratatui::text::Line::from_iter([
        "Bazel server PID: ".bold(),
        state
          .bazel_info
          .server_pid
          .clone()
          .unwrap_or("<elo>".to_string())
          .into(),
      ]),
      ratatui::text::Line::from_iter([
        "Workspace: ".bold(),
        state
          .bazel_info
          .workspace
          .clone()
          .unwrap_or("<elo>".to_string())
          .into(),
      ]),
      ratatui::text::Line::from_iter([
        "Bazel output base: ".bold(),
        state
          .bazel_info
          .output_base
          .clone()
          .unwrap_or("<elo>".to_string())
          .into(),
      ]), // DEBUG
          // ratatui::text::Line::from(format!("{:?}", &state.selected_target)),
    ];

    let bazel_statues = Paragraph::new(statuses)
      .block(Block::new().padding(Padding::symmetric(2, 1)));
    bazel_statues.render(top_row_layout[0], buf);

    let advert = Paragraph::new(vec![
      ratatui::text::Line::from("explore_bzl")
        .alignment(ratatui::layout::HorizontalAlignment::Center),
      ratatui::text::Line::from(format!("v{}", env!("CARGO_PKG_VERSION")))
        .alignment(ratatui::layout::HorizontalAlignment::Center),
    ])
    .block(Block::new().borders(Borders::NONE));
    advert.render(
      top_row_layout[1].centered_vertically(Constraint::Length(4)),
      buf,
    );

    // Bottom Row Layout:
    // * Column 1: Target Selection Window
    // * Column 2: Targets Viewer
    let bottom_row_layout = Layout::default()
      .direction(ratatui::layout::Direction::Horizontal)
      .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
      .spacing(1)
      .split(main_layout[1]);

    // Left side
    let tab_content_block = Block::bordered()
      .padding(Padding::symmetric(2, 1))
      .title(" Targets: ");
    let mut left_list_state = ListState::default().with_selected(
      state.targets.keys().enumerate().find_map(
        |(i, target_label)| match &state.selected_target {
          Some(selected_label) if target_label == selected_label => Some(i),
          _ => None,
        },
      ),
    );

    let left_list =
      List::new(state.targets.keys().map(std::string::String::as_str))
        .highlight_symbol("> ")
        .block(tab_content_block);
    StatefulWidget::render(
      left_list,
      bottom_row_layout[0],
      buf,
      &mut left_list_state,
    );

    // Right side
    let rtab_content_block =
      Block::bordered().padding(Padding::symmetric(2, 1));

    // TODO: Greatly improve
    let target_repr_lines: Vec<String> =
      state
        .selected_target
        .clone()
        .map_or(Vec::default(), |selected_label| {
          state
            .targets_repr
            .get(&selected_label)
            .unwrap_or(&Vec::default())
            .clone()
        });

    let target_overivew_lines: Vec<ratatui::text::Line<'static>> =
      target_repr_lines
        .iter()
        .map(|l| ratatui::text::Line::from(l.clone()))
        .collect();

    let target_overview =
      Paragraph::new(target_overivew_lines).block(rtab_content_block);
    target_overview.render(bottom_row_layout[1], buf);

    let tabs = Tabs::new(vec!["[R]epr", "[A]ttrs", "[C]onfig", "[A]ctions"])
      .select(0)
      .divider(symbols::DOT)
      .padding(" ", " ");
    tabs.render(bottom_row_layout[1] + Offset::new(1, 0), buf);
  }
}
