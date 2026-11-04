import java.io.File

plugins {
    id("com.android.application")
}

// ---------------------------------------------------------------------------
// Signing config
// 来源优先级: 环境变量 ANDROID_KEYSTORE / ANDROID_KEY_ALIAS / ANDROID_KEY_PASSWORD /
// ANDROID_STORE_PASSWORD > gradle.properties 里同名占位项.
// keystore 文件不存在时, release 自动 fallback 到 debug 签名 (APK 仍可 adb install).
// ---------------------------------------------------------------------------
val keystorePath = (System.getenv("ANDROID_KEYSTORE")
    ?: project.findProperty("ANDROID_KEYSTORE") as? String
    ?: "").trim()
val keyAlias = (System.getenv("ANDROID_KEY_ALIAS")
    ?: project.findProperty("ANDROID_KEY_ALIAS") as? String
    ?: "").trim()
val keyPassword = System.getenv("ANDROID_KEY_PASSWORD")
    ?: project.findProperty("ANDROID_KEY_PASSWORD") as? String
    ?: ""
val storePassword = System.getenv("ANDROID_STORE_PASSWORD")
    ?: project.findProperty("ANDROID_STORE_PASSWORD") as? String
    ?: ""

val hasReleaseKeystore = keystorePath.isNotEmpty() &&
    File(keystorePath).isFile &&
    keyAlias.isNotEmpty()

android {
    namespace = "com.example.{{crate_name}}"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.example.{{crate_name}}"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0.0"

        ndk {
            // arm64-v8a: 真机; x86_64: Windows / Intel Mac 上的模拟器.
            // 只有实际存在的 .so 会被打包进 APK.
            abiFilters += listOf("arm64-v8a", "x86_64")
        }

        // NativeActivity 会加载这个名字的 .so. 必须与 Rust cdylib 名一致,
        // 不带 "lib" 前缀, 不带 ".so" 后缀.
        manifestPlaceholders["nativeLibraryName"] = "{{crate_name}}"
    }

    signingConfigs {
        create("release") {
            storeFile = if (hasReleaseKeystore) File(keystorePath) else null
            storePassword = storePassword
            keyAlias = keyAlias
            keyPassword = keyPassword
        }
    }

    buildTypes {
        release {
            signingConfig = if (hasReleaseKeystore) {
                signingConfigs["release"]
            } else {
                println("[WARN] No release keystore configured -> falling back to debug signing.")
                signingConfigs["debug"]
            }
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {
            isDebuggable = true
            isJniDebuggable = true
        }
    }

    // 不使用 CMake / ndk-build — Rust 库由 cargo-ndk 独立编译后放入 jniLibs.
    externalNativeBuild {
        // intentionally left empty
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    // src/main/jniLibs 是 AGP 默认路径, 无需显式声明 sourceSets.

    packaging {
        jniLibs {
            keepDebugSymbols += listOf(
                "*/arm64-v8a/lib{{crate_name}}.so"
            )
        }
    }

    lint {
        abortOnError = false
        checkReleaseBuilds = false
    }
}

dependencies {
    // AndroidX SplashScreen 兼容 (GpuiActivity 持有 splash 直到 native 初始化完成)
    implementation("androidx.core:core-splashscreen:1.0.1")
}
