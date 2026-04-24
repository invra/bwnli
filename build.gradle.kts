plugins {
    kotlin("jvm") version "1.9.24"
    id("io.github.goooler.shadow") version "8.1.8"
}

repositories {
    mavenCentral()
    maven("https://maven.bitwig.com")
}

dependencies {
    compileOnly("com.bitwig:extension-api:25")
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
        resources.srcDirs("bwextension/src/main/resources")
    }
}

val pluginName = "Bitwig-NLI"
val extensionsDir = "${System.getProperty("user.home")}/Documents/Bitwig Studio/Extensions"

tasks {
    jar { enabled = false }

    shadowJar {
        archiveBaseName.set(pluginName)
        archiveClassifier.set("")
        archiveVersion.set("")
        archiveExtension.set("bwextension")
        destinationDirectory.set(file(extensionsDir))
        duplicatesStrategy = DuplicatesStrategy.EXCLUDE
    }

    build {
        dependsOn(shadowJar)
    }
}
