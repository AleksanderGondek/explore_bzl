{
  lib,
  buildFHSEnv,
  writeScript,
  alejandra,
  bazelisk,
  cacert,
  cocogitto,
  gcc,
  git,
  helix,
  libz,
  niv,
  statix,
  deadnix,
  protobuf,
  rust,
  rust-analyzer,
} @ args: let
  packages =
    lib.attrsets.attrValues (
      builtins.removeAttrs args ["buildFHSEnv" "writeScript" "lib" "rust"]
    )
    ++ [rust.bin];
in
  (buildFHSEnv {
    name = "explore_bzl-shell";

    targetPkgs = _: (packages);

    runScript = writeScript "explore_bzl-shell-init.sh" ''
      shellHooksPath=$(mktemp --suffix=explore_bzl-shell.bazelrc)
      cat <<EOF > $shellHooksPath
        export CARGO_HOME="''${PWD}/.cache/.cargo"
        mkdir -p ''${PWD}/.cache/.cargo
        # Just using an 'alias=...'
        # will not work for binaries like starpls, that execute items
        # directly from path.
        mkdir -p .bazelisk-bin
        ln -f -s ${bazelisk}/bin/bazelisk .bazelisk-bin/bazel
        export PATH="$(realpath .bazelisk-bin)/:$PATH"
      EOF

      exec bash --rcfile $shellHooksPath
    '';
  })
  .env
