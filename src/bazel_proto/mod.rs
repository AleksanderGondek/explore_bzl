// No influence over structure of generated code
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
pub mod analysis {
  #[cfg(feature = "bazel")]
  pub use ::bazel_proto::analysis::*;
  #[cfg(not(feature = "bazel"))]
  include!(concat!(env!("OUT_DIR"), "/analysis.rs"));
}
pub mod blaze_query {
  #[cfg(feature = "bazel")]
  pub use ::bazel_proto::blaze_query::*;
  #[cfg(not(feature = "bazel"))]
  include!(concat!(env!("OUT_DIR"), "/blaze_query.rs"));
}
pub mod stardoc_output {
  #[cfg(feature = "bazel")]
  pub use ::bazel_proto::stardoc_output::*;
  #[cfg(not(feature = "bazel"))]
  include!(concat!(env!("OUT_DIR"), "/stardoc_output.rs"));
}
