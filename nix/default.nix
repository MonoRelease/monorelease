{
  nixpkgs,
  rust-overlay,
  ...
}@inputs:
let
  inherit (nixpkgs) lib;
  forAllSystems = lib.genAttrs lib.systems.flakeExposed;

  mkPkgs =
    system:
    import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };

  imports =
    system:
    inputs
    // rec {
      inherit system;
      pkgs = mkPkgs system;
      rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
    };

  forAllSystemsImport =
    file:
    forAllSystems (
      system: (import file ((imports system) // { shared = import ./shared (imports system); }))
    );
in
{
  devShells = forAllSystemsImport ./devShells;
  formatter = forAllSystemsImport ./formatter;
  apps = forAllSystemsImport ./apps;
}
