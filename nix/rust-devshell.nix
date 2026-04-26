{
  mkShell,
  inputs,
  stdenv,
  trunk,
  callPackage
}:
let
  inherit (inputs)fenix;
  toolchain = with fenix.packages.${stdenv.system}; combine [
    latest.toolchain
    targets.wasm32-unknown-unknown.latest.rust-std
  ];
  wasm-bindgen-cli = callPackage ./wasm-bindgen-cli.nix {};
  # cargo-leptos = callPackage ./cargo-leptos.nix {};
in
mkShell
{
  buildInputs = [
    # cargo-leptos
    trunk
    toolchain
    wasm-bindgen-cli
  ];
}
