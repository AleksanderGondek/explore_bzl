use std::collections::BTreeMap;

use prost::Message;
use tokio::{process::Command, sync::mpsc};

use crate::{
  Result,
  bazel_proto::blaze_query::Target,
  event::{BazelCommand, BazelQuery, Event},
  model::BazelInfo,
};

pub struct BazelTask {
  command: BazelCommand,
  sender: mpsc::UnboundedSender<Event>,
}

impl BazelTask {
  #[must_use]
  pub fn new(
    command: BazelCommand,
    sender: mpsc::UnboundedSender<Event>,
  ) -> Self {
    Self { command, sender }
  }

  pub async fn run(&self) -> Result<()> {
    // TODO: Each command, separate handler
    if let BazelCommand::Info = &self.command {
      // No unwrap please
      let Ok(output) = Command::new("bazel").arg("info").output().await else {
        // TODO: Better eror handling
        return Err(crate::Error::Imaginary);
      };
      // Tmp structure
      let stdout = String::from_utf8_lossy(&output.stdout).to_string();
      // TODO: Handle the result of sending
      let _ = self.sender.send(Event::BazelResponse(
        crate::event::BazelCmdResponse::Info(Box::new(into_bazel_info(
          &stdout,
        ))),
      ));
    }
    if let BazelCommand::QueryForRepr(label) = &self.command {
      let Ok(output) = Command::new("bazel")
        .args(["query", "--output=build", label])
        .output()
        .await
      else {
        return Err(crate::Error::Imaginary);
      };
      let starlark_repr = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
      let _ = self.sender.send(Event::BazelResponse(
        crate::event::BazelCmdResponse::QueryForRepr(Box::new((
          *label.clone(),
          starlark_repr,
        ))),
      ));
    }
    if let BazelCommand::Query(crate::event::BazelQuery::Targets) =
      &self.command
    {
      let Ok(output) = Command::new("bazel")
        .args(["query", "--output=proto", "//..."])
        .output()
        .await
      else {
        return Err(crate::Error::Imaginary);
      };

      let x =
        crate::bazel_proto::blaze_query::QueryResult::decode(&*output.stdout)?
          .target
          .iter()
          .filter_map(|target| {
            target
              .rule
              .as_ref()
              .map(|r| (r.name.clone(), target.clone()))
          })
          .collect::<BTreeMap<String, Target>>();

      let _ = self.sender.send(Event::BazelResponse(
        crate::event::BazelCmdResponse::Query(Box::new(x)),
      ));
    }
    if let BazelCommand::Cquery(BazelQuery::Target(target)) = &self.command {
      let Ok(output) = Command::new("bazel")
        .args(["cquery", "--output=proto", target])
        .output()
        .await
      else {
        return Err(crate::Error::Imaginary);
      };

      let r: crate::bazel_proto::analysis::CqueryResult =
        crate::bazel_proto::analysis::CqueryResult::decode(&*output.stdout)?;
      let _ = self.sender.send(Event::BazelResponse(
        crate::event::BazelCmdResponse::Cquery(Box::new(BTreeMap::from_iter(
          [(*target.clone(), r)],
        ))),
      ));
    }

    Ok(())
  }
}

fn into_bazel_info(text: &str) -> BazelInfo {
  let mut response = BazelInfo::default();
  for line in text.lines() {
    match line.split_at(line.find(':').unwrap_or(line.len())) {
      ("bazel_bin", c) => response.bazel_bin = Some(c.to_string()),
      ("bazel_genfiles", c) => response.bazel_genfiles = Some(c.to_string()),
      ("bazel_testlogs", c) => response.bazel_testlogs = Some(c.to_string()),
      ("character_encoding", c) => {
        response.character_encoding = Some(c.to_string());
      }
      ("command_log", c) => response.command_log = Some(c.to_string()),
      ("commited_heap_size", c) => {
        response.commited_heap_size = Some(c.to_string());
      }
      ("execution_root", c) => response.execution_root = Some(c.to_string()),
      ("gc_count", c) => response.gc_count = Some(c.to_string()),
      ("gc_time", c) => response.gc_time = Some(c.to_string()),
      ("install_base", c) => response.install_base = Some(c.to_string()),
      ("java_home", c) => response.java_home = Some(c.to_string()),
      ("java_runtime", c) => response.java_runtime = Some(c.to_string()),
      ("java_vm", c) => response.java_vm = Some(c.to_string()),
      ("local_repository", c) => {
        response.local_repository = Some(c.to_string());
      }
      ("max_heap_size", c) => response.max_heap_size = Some(c.to_string()),
      ("output_base", c) => response.output_base = Some(c.to_string()),
      ("output_path", c) => response.output_path = Some(c.to_string()),
      ("package_path", c) => response.package_path = Some(c.to_string()),
      ("release", c) => response.release = Some(c.to_string()),
      ("repository_cache", c) => {
        response.repository_cache = Some(c.to_string());
      }
      ("server_log", c) => response.server_log = Some(c.to_string()),
      ("server_pid", c) => response.server_pid = Some(c.to_string()),
      ("used_heap_size", c) => response.used_heap_size = Some(c.to_string()),
      ("workspace", c) => response.workspace = Some(c.to_string()),
      _ => (), // TODO: Warn on unknown entry?
    }
  }
  response
}
