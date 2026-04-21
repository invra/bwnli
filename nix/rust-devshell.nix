{
  mkShell,
  rustc,
  cargo,
  rustfmt,
  clippy
}:
mkShell {
  buildInputs = [
    rustc
    cargo
    rustfmt
    clippy
  ];
}
