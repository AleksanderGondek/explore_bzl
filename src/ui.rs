use ratatui::{
  buffer::Buffer,
  layout::{Constraint, Layout, Offset, Rect},
  style::Stylize,
  symbols,
  widgets::{
    Block, List, ListState, Padding, Paragraph, StatefulWidget, Tabs, Widget,
  },
};

use crate::model::Model;

// TODO: In future move this into ui.rs and kill this module
#[derive(Debug, Default)]
pub struct Ui {}

impl StatefulWidget for Ui {
  type State = Model;
  fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
    let layout = Layout::default()
      .direction(ratatui::layout::Direction::Vertical)
      .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
      .spacing(1)
      .split(area);

    let bottom_layout = Layout::default()
      .direction(ratatui::layout::Direction::Horizontal)
      .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
      .spacing(1)
      .split(layout[1]);

    let statuses: Vec<ratatui::text::Line<'static>> = vec![
            ratatui::text::Line::from_iter([
                "Bazel binary: ".bold(),
                "/home/agondek/projects/aleksandergondek/rules_cc_hdrs_map/.bazelisk-bin/bazel"
                    .into(),
            ]),
            ratatui::text::Line::from_iter(["Bazel version: ".bold(), state.bazel_info.release.clone().unwrap_or("<ELO>".to_string()).into()]), //TODO: This is ugly, needs fixing
            ratatui::text::Line::from_iter(["Bazel server PID: ".bold(), state.bazel_info.server_pid.clone().unwrap_or("<elo>".to_string()).into()]),
            ratatui::text::Line::from_iter([
                "Workspace: ".bold(),
                state.bazel_info.workspace.clone().unwrap_or("<elo>".to_string()).into(),
            ]),
            // DEBUG
            ratatui::text::Line::from(format!("{:?}", &state.selected_target)),
        ];

    let top_paragraph = Paragraph::new(statuses)
      .block(Block::new().padding(Padding::symmetric(2, 1)));
    top_paragraph.render(layout[0], buf);

    // Left side
    let tab_content_block = Block::bordered().padding(Padding::symmetric(2, 1));
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
      bottom_layout[0],
      buf,
      &mut left_list_state,
    );

    let tabs = Tabs::new(vec!["[P]ackage browser", "[F]uzzy search", "[R]aw"])
      .select(0)
      .divider(symbols::DOT)
      .padding(" ", " ");
    tabs.render(bottom_layout[0] + Offset::new(1, 0), buf);

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
    target_overview.render(bottom_layout[1], buf);

    let tabs = Tabs::new(vec!["[Q]uery", "[C]query", "[A]query"])
      .select(0)
      .divider(symbols::DOT)
      .padding(" ", " ");
    tabs.render(bottom_layout[1] + Offset::new(1, 0), buf);
  }
}
