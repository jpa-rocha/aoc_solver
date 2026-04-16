{
  age,
  git,
  mkShell,
  scripts,
  self,
  sops,
  system,
  cargo,
  rust-analyzer,
  rustc,
  ...
}:
{
  default = mkShell {
    packages = [
      age
      cargo
      git
      rust-analyzer
      scripts.lint
      sops
      rustc
    ];

    inherit (self.checks.${system}.pre-commit-check) shellHook;
  };
}
