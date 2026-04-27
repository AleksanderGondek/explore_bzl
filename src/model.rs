use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct BazelInfo {
  pub binary: Option<PathBuf>,
  pub server_pid: Option<u32>,
  pub version: Option<String>,
  pub workspace: Option<PathBuf>,
}

#[derive(Debug, Default)]
pub struct Model {
  pub bazel_info: BazelInfo,
  pub should_quit: bool,
}
