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
      .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
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
        ];

    let top_paragraph = Paragraph::new(statuses)
      .block(Block::new().padding(Padding::symmetric(2, 1)));
    top_paragraph.render(layout[0], buf);

    // Left side
    let tab_content_block = Block::bordered().padding(Padding::symmetric(2, 1));
    let mut left_list_state = ListState::default();
    if !state.targets.is_empty() {
      left_list_state.select(state.targets_selection);
    }

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
    let target_repr: Vec<String>;
    if let Some(target) = state.selected_target() {
      if let Some(repr) = &target.starlark_repr {
        target_repr = repr.lines().map(std::convert::Into::into).collect();
      } else {
        target_repr = Vec::default();
      }
    } else {
      target_repr = Vec::default();
    }

    let target_overivew_lines: Vec<ratatui::text::Line<'static>> = target_repr
      .iter()
      .map(|l| ratatui::text::Line::from(l.clone()))
      .collect();
    // let target_overivew_lines: Vec<ratatui::text::Line<'static>> = vec![
    //         r"platform(".into(),
    //         r#"  name = "x86_64_linux_remote","#.into(),
    //         r#"  visibility = ["//visibility:public"],"#.into(),
    //         r#"  constraint_values = ["@platforms//os:linux", "@platforms//cpu:x86_64"],"#.into(),
    //         r#"  exec_properties = {"OSFamily": "Linux", "container-image": "docker://harbor.apps.morrigna.rules-nix.build/explore-bzl/ash-bash-coreutils-i686-cc-x86_64-cc:myl0xwv1z442sc5ci982qny9lb0c0giv"},"#.into(),
    //         r")".into(),
    //     ];
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
