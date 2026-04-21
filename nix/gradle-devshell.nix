{
  mkShell,
  gradle,
  kotlin,
  temurin-bin-17,
  kotlin-language-server
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
