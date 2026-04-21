{
  description = "Bitwig-NLI - A tool to get Novation LPProMk3 MIDI Info to a Website for OBS Scene";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
  let
    forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
  in
  {
    devShells = forAllSystems (system: let
      pkgs = import nixpkgs { inherit system; };
    in rec {
      program = pkgs.callPackage ./nix/rust-devshell.nix {};
      bitwig = pkgs.callPackage ./nix/gradle-devshell.nix {};
      default = program;
    });

    packages = forAllSystems (system: let
      pkgs = import nixpkgs { inherit system; };
    in rec {
      program = pkgs.callPackage ./nix/rust-build.nix {};
      default = program;
    });
  };
}
