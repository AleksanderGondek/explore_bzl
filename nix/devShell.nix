{
  lib,
  mkShell,
  alejandra,
  cacert,
  cocogitto,
  gcc,
  git,
  helix,
  niv,
  statix,
  deadnix,
  rust,
  rust-analyzer,
} @ args: let
  packages =
    lib.attrsets.attrValues (
      builtins.removeAttrs args ["mkShell" "lib" "rust"]
    )
    ++ [rust.bin];
in
  mkShell {
    inherit packages;
    name = "explore_bzl-shell";

    shellHook = ''
      export CARGO_HOME="''${PWD}/.cache/.cargo"
      mkdir -p ''${CARGO_HOME}
    '';
  }
