{nixpkgs}: let
  customCallPackage = extraPkgs:
    nixpkgs.lib.callPackageWith (
      nixpkgs
      // extraPkgs
      // {
        callPackage = nixpkgs.lib.callPackageWith (
          nixpkgs // extraPkgs
        );
      }
    );
in rec {
  rust = (customCallPackage {}) ./rust.nix {};
  devShell = (customCallPackage {inherit rust;}) ./devShell.nix {};
  nixosDevShell = (customCallPackage {inherit rust;}) ./nixosDevShell.nix {};
}
