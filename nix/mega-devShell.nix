{
  typos,
  mkShell,
  callPackage,  
  inputs
}:
mkShell {
  # merge in both shells
  inputsFrom = [
    (callPackage ./rust-devshell.nix { inherit inputs; })
    (callPackage ./gradle-devshell.nix { inherit inputs; })
  ];
  buildInputs = [
    typos
  ];
}
