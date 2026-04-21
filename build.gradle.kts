plugins {
    kotlin("jvm") version "1.9.24"
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
}

kotlin {
    jvmToolchain(17)
}

sourceSets {
    main {
        kotlin.srcDirs("bwextension/src/main/kotlin")
    }
}

tasks.jar {
    manifest {
        attributes["Main-Class"] = "net.invra.bwnli.MainKt"
    }
}
