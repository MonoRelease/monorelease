{
  pkgs,
  shared,
  ...
}:
{
  default = pkgs.mkShell {
    inherit (shared.preCommit) shellHook;
    inherit (shared.core) nativeBuildInputs buildInputs env;
  };

  docs = pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      pnpm
    ];
  };
}
