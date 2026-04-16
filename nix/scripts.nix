{ pkgs, cargo, ... }:
{
  lint = pkgs.writeShellScriptBin "lint" ''
    set -euo pipefail
    ${pkgs.nix}/bin/nix flake check
  '';

}
