{ pkgs, rustfmt, ... }:
{
  projectRootFile = "flake.nix";

  programs = {
    deadnix.enable = true;
    ruff.enable = true;
    nixfmt.enable = true;
    prettier.enable = true;
    shfmt.enable = true;
    templ.enable = true;
    rustfmt.enable = true;
  };

  settings = {
    global.excludes = [
      ".venv/*"
    ];

    formatter = {
      rustfmt = {
        command = "${rustfmt}/bin/rustfmt";
        options = [ "--base-formatter=${rustfmt}/bin/rustfmt" ];
        includes = [ "**/*.rs" ];
      };

      prettier = {
        package = pkgs.nodePackages.prettier;
        excludes = [
        ];
      };
    };
  };
}
