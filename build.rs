fn main() {
  protobuf_codegen::Codegen::new()
    .protoc()
    .includes(["src/bazel_proto"])
    .input("src/bazel_proto/src/main/protobuf/build.proto")
    .out_dir("src/bazel_proto")
    .run_from_script();
}
