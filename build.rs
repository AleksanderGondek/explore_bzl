use std::io::Result;

fn main() -> Result<()> {
  prost_build::compile_protos(
    &[
      "./src/bazel_proto/src/main/protobuf/build.proto",
      "./src/bazel_proto/src/main/protobuf/stardoc_output.proto",
    ],
    &["./src/bazel_proto"],
  )
}
