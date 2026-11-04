// Root build.gradle.kts — GPUI Kit Mobile Android Template.
//
// 这个工程仅负责将预编译的 Rust 共享库 (lib{{crate_name}}.so)
// 打包进 APK. Rust 端通过 cargo-ndk 单独编译, 产物放入
// app/src/main/jniLibs/<abi>/ 下, Gradle 会自动打包.
//
// 构建步骤:
//   1. cargo ndk -t arm64-v8a -o android/gradle/app/src/main/jniLibs build --release
//   2. cd android/gradle && ./gradlew assembleDebug
//   3. adb install -r app/build/outputs/apk/debug/app-debug.apk

buildscript {
    repositories {
        google()
        mavenCentral()
    }
    dependencies {
        classpath("com.android.tools.build:gradle:9.1.0")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.22")
    }
}

tasks.register("clean", Delete::class) {
    delete(rootProject.layout.buildDirectory)
}
