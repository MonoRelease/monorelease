{ pkgs, shared, ... }:
let
  ci = pkgs.stdenv.mkDerivation {
    name = "ci";

    nativeBuildInputs = shared.core.nativeBuildInputs ++ [
      pkgs.makeWrapper
    ];
    buildInputs = shared.core.buildInputs;

    dontUnpack = true;

    installPhase = ''
      mkdir -p $out/bin

      cat > $out/bin/ci-raw <<'EOF'
      #!/usr/bin/env bash
      set -euo pipefail

      echo "==== Fetching cargo dependencies ===="
      # pre-commit runs the Cargo checks, such as Clippy, in offline mode.
      # This means that they expect the dependencies to already be installed before running.
      cargo fetch

      echo ""
      echo "==== Setting up pre-commit environment ===="
      ${shared.preCommit.shellHook}

      echo ""
      echo "==== Running pre-commit checks ===="
      # It needs to run against a different cargo target to not mess up the cargo cache for the later "cargo test" run
      CARGO_TARGET_DIR="target/pre-commit" pre-commit run -a

      echo ""
      echo "==== Running cargo tests ===="
      cargo test
      EOF

      chmod +x $out/bin/ci-raw

      makeWrapper $out/bin/ci-raw $out/bin/ci \
        --prefix PATH : "${pkgs.lib.makeBinPath shared.core.nativeBuildInputs}" \
        --set PKG_CONFIG_PATH "$PKG_CONFIG_PATH" \
        --set LD_LIBRARY_PATH "${shared.core.env.LD_LIBRARY_PATH}"
    '';
  };

  mkWebDerivation =
    { name, command }:
    pkgs.stdenv.mkDerivation {
      inherit name;

      nativeBuildInputs = shared.website.nativeBuildInputs ++ [
        pkgs.makeWrapper
      ];

      dontUnpack = true;

      installPhase = ''
        mkdir -p $out/bin

        cat > $out/bin/${name}-raw <<'EOF'
        #!/usr/bin/env bash
        set -euo pipefail

        cd website
        pnpm install
        ${command}

        EOF

        chmod +x $out/bin/${name}-raw

        makeWrapper $out/bin/${name}-raw $out/bin/${name} \
          --prefix PATH : "${pkgs.lib.makeBinPath shared.website.nativeBuildInputs}" \
          --set PKG_CONFIG_PATH "$PKG_CONFIG_PATH" \
      '';
    };

  webStart = mkWebDerivation {
    name = "web-start";
    command = "pnpm docusaurus start";
  };

  webBuild = mkWebDerivation {
    name = "web-build";
    command = "pnpm docusaurus build";
  };
in
{
  ci = {
    type = "app";
    program = "${ci}/bin/ci";
  };

  webStart = {
    type = "app";
    program = "${webStart}/bin/web-start";
  };

  webBuild = {
    type = "app";
    program = "${webBuild}/bin/web-build";
  };
}
