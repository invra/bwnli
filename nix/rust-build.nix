{
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "bwnli";
  version = "0.1.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };
}
