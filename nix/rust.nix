{
  makeRustPlatform,
  pkgsCross,
  rust-bin,
  ...
}: let
  bin = rust-bin.stable.latest.default.override {
    extensions = ["rust-src"];
    targets = ["x86_64-unknown-linux-musl"];
  };
  platform = makeRustPlatform {
    cargo = bin;
    rustc = bin;
  };
  platform-musl = pkgsCross.musl64.makeRustPlatform {
    cargo = bin;
    rustc = bin;
  };
in {
  inherit bin platform platform-musl;
}
