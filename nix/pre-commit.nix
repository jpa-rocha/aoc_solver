{
  nixfmt-rfc-style,
  prettier,
  trufflehog,
  rustfmt,
  ...
}:
{
  src = ./.;
  excludes = [
    "flake.lock"
    ".venv/.+"
  ];
  hooks = {
    check-yaml.enable = true;
    convco.enable = true;
    deadnix.enable = true;
    ripsecrets.enable = true;
    shellcheck.enable = true;
    rustfmt.enable = true;

    nixfmt-rfc-style = {
      enable = true;
      package = nixfmt-rfc-style;
    };

    trufflehog = {
      enable = true;
      package = trufflehog;
    };

    prettier = {
      enable = true;
      package = prettier;
    };

    rustfmt = {
      enable = true;
      entry = "${rustfmt}/bin/rustfmt";
      files = "\\.rs$";
    };
  };
}
