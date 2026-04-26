{
  description = "Bitwig-NLI - A tool to get Novation LPProMk3 MIDI Info to a Website for OBS Scene";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, ... }@inputs:
  let
    forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
  in
  {
    devShells = forAllSystems (system: let
      pkgs = import nixpkgs { inherit system; };
    in {
      program = pkgs.callPackage ./nix/rust-devshell.nix { inherit inputs; };
      bitwig = pkgs.callPackage ./nix/gradle-devshell.nix { inherit inputs; };
      default = pkgs.callPackage ./nix/mega-devShell.nix { inherit inputs; };
    });

    packages = forAllSystems (system: let
      pkgs = import nixpkgs { inherit system; };
    in rec {
      program = pkgs.callPackage ./nix/rust-build.nix { inherit inputs; };
      default = program;
    });
  };
}
