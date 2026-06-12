use std::collections::VecDeque;

use ratatui::{
  buffer::Buffer,
  layout::{Constraint, Layout, Offset, Rect},
  style::Stylize,
  symbols,
  text::Span,
  widgets::{
    Block, Borders, List, ListState, Padding, Paragraph, StatefulWidget, Tabs,
    Widget,
  },
};

use crate::bazel_proto::blaze_query::attribute::Discriminator;
use crate::model::{Model, Pane};

trait Spanify {
  fn spanify(&self) -> Span<'static>;
}

impl<T: Clone + ToString> Spanify for Option<T> {
  fn spanify(&self) -> Span<'static> {
    self
      .clone()
      .map_or_else(|| "loading...".to_string(), |e| e.to_string())
      .into()
  }
}

trait NamingisDiffcult {
  fn stringfy(&self) -> Vec<String>;
}

// TODO(agondek): refactor
impl NamingisDiffcult for crate::bazel_proto::analysis::CqueryResult {
  fn stringfy(&self) -> Vec<String> {
    let mut result: Vec<String> = Vec::default();
    for cfg in &self.configurations {
      result.push(format!("\"id\": \"{}\"", cfg.id));
      result.push(format!("\"mnemonic\": \"{}\"", cfg.mnemonic));
      result.push(format!("\"platform_name\": \"{}\"", cfg.platform_name));
      result.push(format!("\"checksum\":\"{}\"", cfg.checksum));
      result.push(format!("\"is_tool\": \"{}\"", cfg.is_tool));

      // cfg.fragments has nothing of interest
      for fragment_option in &cfg.fragment_options {
        result.push(format!("{} {{", fragment_option.name));
        for option in &fragment_option.options {
          result.push(format!(
            "  \"{}\": \"{}\"",
            option.name.clone().unwrap_or("???".to_string()),
            option.value()
          ));
        }
        result.push("},".to_string());
      }
    }
    result
  }
}

// TODO(agondek): refactor
#[allow(clippy::too_many_lines)]
impl NamingisDiffcult for crate::bazel_proto::blaze_query::Attribute {
  fn stringfy(&self) -> Vec<String> {
    let mut value: VecDeque<String> = match self.r#type() {
      Discriminator::Integer => {
        VecDeque::from_iter([format!("{}", self.int_value())])
      }
      Discriminator::Boolean => {
        VecDeque::from_iter([self.string_value().to_string()])
      }
      Discriminator::String | Discriminator::Label | Discriminator::Output => {
        VecDeque::from_iter([format!("\"{}\"", self.string_value())])
      }
      Discriminator::StringList
      | Discriminator::LabelList
      | Discriminator::OutputList
      | Discriminator::DistributionSet => {
        // Ugly
        let mut result = VecDeque::default();
        if !self.string_list_value.is_empty() {
          result.push_back("[".to_string());
          result.extend(
            self.string_list_value.iter().map(|v| format!("  \"{v}\",")),
          );
          result.push_back("],".to_string());
        }
        result
      }
      // TODO(agondek): Improve
      Discriminator::License => self
        .license
        .as_ref()
        .map_or(VecDeque::default(), |lic| {
          VecDeque::from_iter([format!("{lic:#?}")])
        })
        .clone(),
      Discriminator::StringDict => {
        // Ugly
        let mut result = VecDeque::default();
        if !self.string_dict_value.is_empty() {
          result.push_back("{".to_string());
          result.extend(self.string_dict_value.iter().map(|entry| {
            format!("  \"{0}\": \"{1}\",", entry.key, entry.value)
          }));
          result.push_back("},".to_string());
        }
        result
      }
      // TODO(agondek): Improve
      Discriminator::FilesetEntryList => self
        .fileset_list_value
        .iter()
        .map(|f| format!("{f:#?}"))
        .collect(), // Prettier?
      Discriminator::LabelListDict | Discriminator::StringListDict => {
        // Ugly
        let mut result = VecDeque::default();
        if !self.string_list_dict_value.is_empty() {
          result.push_back("{".to_string());
          result.extend(self.string_list_dict_value.iter().flat_map(|entry| {
            // Ugly^2
            let mut result = VecDeque::default();
            result.push_back(format!("  \"{0}\": [", entry.key));
            if !entry.value.is_empty() {
              result
                .extend(entry.value.iter().map(|v| format!("    \"{v}\",")));
            }
            result.push_back("  ],".to_string());
            result
          }));
          result.push_back("},".to_string());
        }
        result
      }
      // TODO(agondek): Improve
      Discriminator::Tristate => {
        VecDeque::from_iter([format!("{:#?}", self.tristate_value())])
      } // Prettier?
      Discriminator::IntegerList => {
        // Ugly
        let mut result = VecDeque::default();
        if !self.int_list_value.is_empty() {
          result.push_back("[".to_string());
          result.extend(self.int_list_value.iter().map(|v| format!("{v},")));
          result.push_back("],".to_string());
        }
        result
      }
      // TODO(agondek): Improve
      Discriminator::Unknown => VecDeque::from_iter(["Unknown!".to_string()]),
      // TODO(agondek): Improve
      Discriminator::LabelDictUnary => {
        VecDeque::from_iter([format!("{:#?}", self.label_dict_unary_value)])
      }
      // TODO(agondek): Improve (Deeply nested fun)
      Discriminator::SelectorList => {
        VecDeque::from_iter([format!("{:#?}", self.selector_list)])
      }
      Discriminator::LabelKeyedStringDict => {
        // Ugly
        let mut result = VecDeque::default();
        if !self.label_keyed_string_dict_value.is_empty() {
          result.push_back("{".to_string());
          result.extend(self.label_keyed_string_dict_value.iter().map(
            |entry| format!("  \"{0}\": \"{1}\",", entry.key, entry.value),
          ));
          result.push_back("},".to_string());
        }
        result
      }
      // TODO(agondek): Improve (Deeply nested fun)
      Discriminator::DeprecatedStringDictUnary => {
        VecDeque::from_iter([format!(
          "{:#?}",
          self.deprecated_string_dict_unary_value
        )])
      } // Prettier?
    };

    let mut response = Vec::default();
    if let Some(front) = value.pop_front() {
      response.push(format!("{} = {}", self.name, front));
      response.extend(value);
    }
    response
  }
}

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
          .unwrap() // TODO: Handling of bazel binary not being found
          .into(),
      ]),
      ratatui::text::Line::from_iter([
        "Bazel version: ".bold(),
        state.bazel_info.release.spanify(),
      ]), //TODO: This is ugly, needs fixing
      ratatui::text::Line::from_iter([
        "Bazel server PID: ".bold(),
        state.bazel_info.server_pid.spanify(),
      ]),
      ratatui::text::Line::from_iter([
        "Workspace: ".bold(),
        state.bazel_info.workspace.spanify(),
      ]),
      ratatui::text::Line::from_iter([
        "Bazel output base: ".bold(),
        state.bazel_info.output_base.spanify(),
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

    // TODO: This is a nasty
    let target_repr_lines: Vec<String>;
    let target_overview_lines: Vec<ratatui::text::Line<'static>>;
    match state.selected_pane {
      Pane::StarlarkRepr => {
        target_repr_lines = state.selected_target.clone().map_or(
          Vec::default(),
          |selected_label| {
            state
              .targets_repr
              .get(&selected_label)
              .unwrap_or(&Vec::default())
              .clone()
          },
        );

        target_overview_lines = target_repr_lines
          .iter()
          .map(|l| ratatui::text::Line::from(l.clone()))
          .collect();
      }
      Pane::Attributes => {
        target_repr_lines = state.selected_target.clone().map_or(
          Vec::default(),
          |selected_label| {
            state
              .targets
              .get(&selected_label)
              .unwrap()
              .rule
              .as_ref()
              .unwrap()
              .attribute
              .iter()
              .flat_map(NamingisDiffcult::stringfy)
              .collect()
          },
        );

        target_overview_lines = target_repr_lines
          .iter()
          .map(|l| ratatui::text::Line::from(l.clone()))
          .collect();
      }
      Pane::Config => {
        // target_repr_lines = vec!["Config!".to_string()];
        target_repr_lines = state.selected_target.clone().map_or(
          Vec::default(),
          |selected_label| {
            //TODO: Improve
            if !state.targets_cquery.contains_key(&selected_label) {
              return vec!["Loading...".to_string()];
            }
            // TODO(agondek): Improve
            state
              .targets_cquery
              .get(&selected_label)
              .unwrap()
              .stringfy()
            // state
            //   .targets_cquery
            //   .get(&selected_label)
            //   .unwrap()
            //   .configurations
            //   .iter()
            //   .map(|cfg| format!("{:#?}", cfg.fragments))
            //   .collect()
          },
        );

        target_overview_lines = target_repr_lines
          .iter()
          .map(|l| ratatui::text::Line::from(l.clone()))
          .collect();
      }
      Pane::Actions => {
        target_repr_lines = vec!["Actions!".to_string()];

        target_overview_lines = target_repr_lines
          .iter()
          .map(|l| ratatui::text::Line::from(l.clone()))
          .collect();
      }
    }

    let target_overview =
      Paragraph::new(target_overview_lines).block(rtab_content_block);
    target_overview.render(bottom_row_layout[1], buf);

    let tabs = Tabs::new(vec!["[R]epr", "A[t]trs", "[C]onfig", "[A]ctions"])
      .select(Into::<usize>::into(state.selected_pane.clone()))
      .divider(symbols::DOT)
      .padding(" ", " ");
    tabs.render(bottom_row_layout[1] + Offset::new(1, 0), buf);
  }
}
