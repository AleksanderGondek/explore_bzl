use std::{collections::BTreeMap, path::PathBuf, process::Command};

#[derive(Clone, Debug, Default)]
pub struct BazelInfo {
  pub bazel_bin: Option<String>,
  pub bazel_genfiles: Option<String>,
  pub bazel_testlogs: Option<String>,
  pub character_encoding: Option<String>,
  pub command_log: Option<String>,
  pub commited_heap_size: Option<String>,
  pub execution_root: Option<String>,
  pub gc_count: Option<String>,
  pub gc_time: Option<String>,
  pub install_base: Option<String>,
  pub java_home: Option<String>,
  pub java_runtime: Option<String>,
  pub java_vm: Option<String>,
  pub local_repository: Option<String>,
  pub max_heap_size: Option<String>,
  pub output_base: Option<String>,
  pub output_path: Option<String>,
  pub package_path: Option<String>,
  pub release: Option<String>,
  pub repository_cache: Option<String>,
  pub server_log: Option<String>,
  pub server_pid: Option<String>,
  pub used_heap_size: Option<String>,
  pub workspace: Option<String>,
}

// TODO: BazelTargetInfo struct

#[derive(Clone, Debug, Default)]
pub enum Pane {
  #[default]
  StarlarkRepr = 0,
  Attributes = 1,
  Config = 2,
  Actions = 3,
}

impl std::convert::From<Pane> for usize {
  fn from(val: Pane) -> Self {
    match val {
      Pane::StarlarkRepr => 0,
      Pane::Attributes => 1,
      Pane::Config => 2,
      Pane::Actions => 3,
    }
  }
}

#[derive(Debug, Default)]
pub struct Model {
  pub bazel_binary: Option<PathBuf>,
  pub bazel_info: BazelInfo,
  pub should_quit: bool,
  pub selected_pane: Pane,
  // TODO: Move away from String into type Label
  pub selected_target: Option<String>,
  // TODO: definetely unify
  pub targets_cquery:
    BTreeMap<String, crate::bazel_proto::analysis::CqueryResult>,
  // TODO: unify perheps?
  pub targets_repr: BTreeMap<String, Vec<String>>,
  pub targets: BTreeMap<String, crate::bazel_proto::blaze_query::Target>,
}

impl Model {
  #[must_use]
  pub fn init(mut self) -> Self {
    self.bazel_binary = Command::new("sh")
      .args(["-c", "command -v bazel"])
      .output()
      .map(|out| {
        if out.status.success() {
          Some(PathBuf::from(
            String::from_utf8_lossy(&out.stdout).to_string(),
          ))
        } else {
          None
        }
      })
      .ok()
      .flatten();
    self
  }
}
