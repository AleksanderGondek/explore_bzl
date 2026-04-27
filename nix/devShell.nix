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
      # Workaround for jmalloc issues with nixpkgs gcc14
      #  '422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)'
      # See: https://github.com/NixOS/nixpkgs/issues/370494
      # export CFLAGS="-DJEMALLOC_STRERROR_R_RETURNS_CHAR_WITH_GNU_SOURCE"

      mkdir -p ''${CARGO_HOME}
    '';
  }
