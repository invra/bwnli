{
  mkShell,
  rustc,
  cargo,
  rustfmt,
  clippy,
  rust-analyzer
}:
mkShell {
  buildInputs = [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
  ];
}
