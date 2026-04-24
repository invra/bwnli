{
  description = "Bitwig-NLI - A tool to get Novation LPProMk3 MIDI Info to a Website for OBS Scene";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, ... }@inputs:
  let
    forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    overlays = [
      (import inputs.rust-overlay)
    ];
  in
  {
    devShells = forAllSystems (system: let
      pkgs = import nixpkgs { inherit system overlays; };
    in {
      program = pkgs.callPackage ./nix/rust-devshell.nix {};
      bitwig = pkgs.callPackage ./nix/gradle-devshell.nix {};
      default = pkgs.callPackage ./nix/mega-devShell.nix {};
    });

    packages = forAllSystems (system: let
      pkgs = import nixpkgs { inherit system overlays; };
    in rec {
      program = pkgs.callPackage ./nix/rust-build.nix {};
      default = program;
    });
  };
}
