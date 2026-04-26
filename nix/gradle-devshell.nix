{
  mkShell,
  gradle,
  kotlin,
  temurin-bin-17,
  kotlin-language-server,
  ... # to allow inputs to come through
}:
mkShell {
  buildInputs = [
    gradle
    kotlin
    temurin-bin-17
    kotlin-language-server
  ];
  JAVA_HOME = temurin-bin-17;
}
