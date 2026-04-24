{
  typos,
  mkShell,
  callPackage,  
}:
mkShell {
  # merge in both shells
  inputsFrom = [
    (callPackage ./rust-devshell.nix {})
    (callPackage ./gradle-devshell.nix {})
  ];
  buildInputs = [
    typos
  ];
}
