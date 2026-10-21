{
  pkgs,
  git-hooks,
  system,
  rustToolchain,
  ...
}:
{
  preCommit = git-hooks.lib.${system}.run {
    src = ../../.;
    hooks = {
      nixfmt.enable = true; # Nix formatter
      statix.enable = true; # Nix linter
      deadnix.enable = true; # Nix dead code checker
      markdownlint.enable = true; # Markdown
      trufflehog.enable = true; # Secret scanning
      yamllint.enable = true; # YAML linting

      # Rust
      rustfmt = {
        enable = true;
        packageOverrides = {
          rustfmt = rustToolchain;
          cargo = rustToolchain;
        };
      };
      clippy = {
        enable = true;
        packageOverrides = {
          clippy = rustToolchain;
          cargo = rustToolchain;
        };
        settings.denyWarnings = true;
      };
      cargo-sort.enable = true; # Sort Cargo dependencies

      # Misc
      check-added-large-files.enable = true;
      check-case-conflicts.enable = true;
      check-merge-conflicts.enable = true;
    };
  };

  core = {
    nativeBuildInputs = [
      rustToolchain
    ];

    buildInputs = [ ];

    env = {
      LD_LIBRARY_PATH = "";
    };
  };

  website = {
    nativeBuildInputs = with pkgs; [
      pnpm
    ];
  };
}
