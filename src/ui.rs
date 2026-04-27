
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Offset, Rect},
    style::Stylize,
    symbols,
    widgets::{
        Block, List, ListState, Padding, Paragraph, StatefulWidget, Tabs,
        Widget,
    },
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
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
            ratatui::text::Line::from_iter(["Bazel version: ".bold(), "8.5.1".into()]),
            ratatui::text::Line::from_iter(["Bazel server PID: ".bold(), "38388".into()]),
            ratatui::text::Line::from_iter([
                "Workspace: ".bold(),
                "/home/agondek/projects/aleksandergondek/rules_cc_hdrs_map".into(),
            ]),
        ];
        let top_paragraph =
            Paragraph::new(statuses).block(Block::new().padding(Padding::symmetric(2, 1)));
        top_paragraph.render(layout[0], buf);

        // Left side
        let tab_content_block = Block::bordered().padding(Padding::symmetric(2, 1));
        let mut left_list_state = ListState::default();
        left_list_state.select_last();
        let left_list = List::new([
            "a/private/implements (package)",
            "a/private/includes (package)",
            "a/public/implements (package)",
            "a/public/includes (package)",
            "b/private/implements (package)",
            "b/private/includes (package)",
            "b/public/implements (package)",
            "libuuid (package)",
            "simple/cc_so_import (package)",
            "simple/deck (package)",
            "simple/main (package)",
            "simple/messenger (package)",
            "simple/random (package)",
            ":DEV_NULL_PLATFORM",
            "x86_64_linux_remote",
        ])
        .highlight_symbol("> ")
        .block(tab_content_block);
        StatefulWidget::render(left_list, bottom_layout[0], buf, &mut left_list_state);

        let tabs = Tabs::new(vec!["[P]ackage browser", "[F]uzzy search", "[R]aw"])
            .select(0)
            .divider(symbols::DOT)
            .padding(" ", " ");
        tabs.render(bottom_layout[0] + Offset::new(1, 0), buf);

        // Right side
        let rtab_content_block = Block::bordered().padding(Padding::symmetric(2, 1));
        let target_overivew_lines: Vec<ratatui::text::Line<'static>> = vec![
            r"platform(".into(),
            r#"  name = "x86_64_linux_remote","#.into(),
            r#"  visibility = ["//visibility:public"],"#.into(),
            r#"  constraint_values = ["@platforms//os:linux", "@platforms//cpu:x86_64"],"#.into(),
            r#"  exec_properties = {"OSFamily": "Linux", "container-image": "docker://harbor.apps.morrigna.rules-nix.build/explore-bzl/ash-bash-coreutils-i686-cc-x86_64-cc:myl0xwv1z442sc5ci982qny9lb0c0giv"},"#.into(),
            r")".into(),
        ];
        let target_overview = Paragraph::new(target_overivew_lines).block(rtab_content_block);
        target_overview.render(bottom_layout[1], buf);

        let tabs = Tabs::new(vec!["[Q]uery", "[C]query", "[A]query"])
            .select(0)
            .divider(symbols::DOT)
            .padding(" ", " ");
        tabs.render(bottom_layout[1] + Offset::new(1, 0), buf);
    }
}
