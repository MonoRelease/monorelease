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

  website = pkgs.mkShell {
    inherit (shared.website) nativeBuildInputs;
  };
}
