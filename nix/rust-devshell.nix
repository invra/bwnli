{
  trunk,
  mkShell,
  rust-bin,
}:
mkShell {
  buildInputs = [
    trunk
    (rust-bin.stable.latest.complete.override {
      targets = [ "wasm32-unknown-unknown" ];
    })
  ];
}
