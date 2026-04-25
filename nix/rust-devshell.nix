{
  trunk,
  mkShell,
  rust-bin,
  tailwindcss
}:
mkShell {
  buildInputs = [
    trunk
    tailwindcss
    (rust-bin.stable.latest.complete.override {
      targets = [ "wasm32-unknown-unknown" ];
    })
  ];
}
