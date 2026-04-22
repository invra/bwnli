plugins {
    kotlin("jvm") version "1.9.24"
    id("io.github.goooler.shadow") version "8.1.8"
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("io.ktor:ktor-client-websockets:2.3.0")
    implementation("io.ktor:ktor-client-cio:2.3.0")
    implementation("io.ktor:ktor-serialization-kotlinx-json:2.3.0")
    implementation("org.slf4j:slf4j-nop:2.0.9")
}

kotlin {
    jvmToolchain(17)
}

sourceSets {
    main {
        kotlin.srcDirs("bwextension/src/main/kotlin")
    }
}

application {
    mainClass.set("MainKt")
}

tasks {
    jar { enabled = false }
    distZip { enabled = false }
    distTar { enabled = false }
    startScripts { enabled = false }
    build { dependsOn(shadowJar) }
    shadowJar {
        archiveClassifier.set("")
        manifest {
            attributes["Main-Class"] = "MainKt"
        }
    }
}
