use std::collections::BTreeMap;

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

#[derive(Clone, Debug, Default)]
pub struct BazelTarget {
  pub label: String,
  pub starlark_repr: Option<String>,
}

#[derive(Debug, Default)]
pub struct Model {
  pub bazel_info: BazelInfo,
  pub should_quit: bool,
  pub targets: BTreeMap<String, BazelTarget>,
  pub targets_selection: Option<usize>,
}

impl Model {
  #[must_use] 
  pub fn selected_target(&self) -> Option<&BazelTarget> {
    if let Some(i) = self.targets_selection {
      return self.targets.iter().take(i + 1).next_back().map(|(_, t)| t);
    }
    None
  }
}
