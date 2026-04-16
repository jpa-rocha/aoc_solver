{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";
    nixpkgs-stable.url = "github:nixos/nixpkgs/nixos-25.05";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    pre-commit-hooks.url = "github:cachix/git-hooks.nix";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    { self, ... }@inputs:
    let
      eachSystem = inputs.flake-utils.lib.eachDefaultSystem;
    in
    eachSystem (
      system:
      let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;

        rust-analyzer = pkgs.rust-analyzer;

        rustfmt = pkgs.rustfmt;

        rustc = pkgs.rustc;

        cargo = pkgs.cargo;

        scripts = import ./nix/scripts.nix { inherit pkgs cargo; };

        treefmtEval = inputs.treefmt-nix.lib.evalModule pkgs rustfmt ./nix/treefmt.nix;

        preCommit = import ./nix/pre-commit.nix {
          nixfmt-rfc-style = pkgs.nixfmt-rfc-style;
          prettier = pkgs.nodePackages.prettier;
          trufflehog = pkgs.trufflehog;
          rustfmt = rustfmt;
        };
      in
      {
        devShells = import ./nix/devshells.nix {
          inherit
            pkgs
            lib
            scripts
            self
            system
            cargo
            rust-analyzer
            rustfmt
            rustc
            ;
          inherit (pkgs)
            age
            git
            mkShell
            sops
            ;
        };

        checks = {
          formatting = treefmtEval.config.build.check inputs.self;
          pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run preCommit;
        };

        formatter = treefmtEval.config.build.wrapper;
      }
    );
}
