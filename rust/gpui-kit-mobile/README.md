# {{ project-name }} — GPUI Kit Mobile

基于 gpui-kit (GPUI) + gpui-pre-mobile 的 Android 原生应用模板.

## 特性

- **GUI 框架**: [gpui-kit](https://github.com/longbridge/gpui-kit) 组件库 + [gpui-pre-mobile](https://crates.io/crates/gpui-pre-mobile) 移动端适配
- **Android**: NativeActivity + cargo-ndk 交叉编译, 无需 CMake / NDK-build
- **ABI**: 自动选择 —— 真机 `arm64-v8a`, 模拟器 `x86_64`, 详见下文
- **构建**: Gradle (APK) + cargo-ndk (Rust .so), 用 `android/build.bat` / `build.sh` 一键构建
- **日志**: log → logcat, panic → logcat

## 项目结构

```
.
├── Cargo.toml                    # Workspace 根 (单 member)
├── .cargo/config.toml
├── .gitignore
├── cargo-generate.toml
├── README.md
│
├── {{ project-name }}-app/       # Rust crate (cdylib + staticlib + lib)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                # android_main + AppView
│
└── android/                      # Gradle APK 工程
    ├── .cargo/config.toml
    ├── build.bat / build.sh
    └── gradle/
        ├── build.gradle.kts
        ├── settings.gradle.kts
        ├── gradle.properties
        ├── gradle/wrapper/       # gradle-wrapper.jar + properties
        └── app/
            ├── build.gradle.kts
            ├── proguard-rules.pro
            └── src/main/
                ├── AndroidManifest.xml
                └── res/          # mipmap / drawable / values / values-v31
```

## 构建

### Prerequisites

- **Android SDK + NDK** (minSdk 26, compileSdk 34, NDK 推荐 r27+)
- **cargo-ndk**: `cargo install cargo-ndk`
- **Rust target**: `rustup target add aarch64-linux-android`
- **Gradle Wrapper** (已包含)

### 环境变量

<details open>
<summary>Windows (PowerShell)</summary>

```powershell
# 临时 (当前终端有效)
$env:JAVA_HOME      = "<JDK path, e.g. C:\Program Files\Android\Android Studio\jbr>"
$env:ANDROID_HOME   = "<Android SDK path>"
$env:ANDROID_NDK_HOME = "<NDK path, usually under ANDROID_HOME\NDK>"

# 持久化 (用户级, 下次 PowerShell 自动生效)
[Environment]::SetEnvironmentVariable("JAVA_HOME", "<JDK path>", "User")
[Environment]::SetEnvironmentVariable("ANDROID_HOME", "<SDK path>", "User")
[Environment]::SetEnvironmentVariable("ANDROID_NDK_HOME", "<NDK path>", "User")
```
</details>

<details>
<summary>Linux / macOS (bash/zsh)</summary>

```bash
# 临时
export JAVA_HOME="<JDK path>"
export ANDROID_HOME="<Android SDK path>"
export ANDROID_NDK_HOME="<NDK path>"

# 持久化 (~/.bashrc or ~/.zshrc)
echo 'export ANDROID_HOME=$HOME/Android/Sdk' >> ~/.bashrc
echo 'export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/<version>' >> ~/.bashrc
echo 'export PATH=$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH' >> ~/.bashrc
source ~/.bashrc
```
</details>

### 一键构建脚本

| 平台 | 脚本 | 说明 |
|------|------|------|
| Windows | `android/build.bat` | CMD/PowerShell 均可调用 |
| Linux / macOS | `android/build.sh` | 需先 `chmod +x android/build.sh` |

**参数** (两个脚本完全一致):

| 参数 | 说明 |
|------|------|
| `--release` | Release 模式 (体积小, 编译慢) |
| `--clean` | 构建前清理 cargo target + gradle build |
| `--run` | 构建后安装到设备并启动 (默认只构建 APK) |
| `--simulator <avd>` | 启动指定 AVD 模拟器 → 等待 boot 完成 → 安装 → 启动 (隐含 `--run`) |

> 默认只构建 APK, 不安装也不启动; 需要装机运行才加 `--run`.
> ABI 自动选择: 真机 `arm64-v8a`, 模拟器 `x86_64`.

**Windows 示例**:

```powershell
cd android

# 只构建 APK (arm64-v8a), 不装机
.\build.bat

# Release 模式 + 清理
.\build.bat --release --clean

# 装到已连接的 arm64 真机并启动
.\build.bat --run

# 自动启动模拟器并安装运行 (<AVD_NAME> 用 `emulator -list-avds` 查看; 自动编译 x86_64)
.\build.bat --simulator <AVD_NAME>
```

**Linux / macOS 示例**:

```bash
cd android

./build.sh
./build.sh --release --clean
./build.sh --run
./build.sh --simulator <AVD_NAME>
```

**手动两步** (完全脱离脚本):

```bash
# 1. 编译 Rust .so (真机用 arm64-v8a, 模拟器用 x86_64)
cargo ndk -t arm64-v8a --platform 31 build --release \
    -o android/gradle/app/src/main/jniLibs

# 2. 组装 APK
cd android/gradle
./gradlew assembleDebug
```

APK 位于 `android/gradle/app/build/outputs/apk/{debug|release}/`.

### Release 签名

模板会从**环境变量**或 `android/gradle/gradle.properties` 读取 release 签名配置.
两种方式任选一种; **留空时 release 会自动 fallback 到 debug 签名**,
APK 仍可直接 `adb install`.

推荐用环境变量 (密码不会落盘):

```powershell
# Windows PowerShell (仅当前会话)
$env:ANDROID_KEYSTORE = "D:\path\to\release.jks"
$env:ANDROID_KEY_ALIAS = "my-key"
$env:ANDROID_KEY_PASSWORD = "YOUR_PASSWORD"
$env:ANDROID_STORE_PASSWORD = "YOUR_PASSWORD"
```

```bash
# Linux / macOS (仅当前会话)
export ANDROID_KEYSTORE="$HOME/release.jks"
export ANDROID_KEY_ALIAS="my-key"
export ANDROID_KEY_PASSWORD="YOUR_PASSWORD"
export ANDROID_STORE_PASSWORD="YOUR_PASSWORD"
```

或写进 `android/gradle/gradle.properties` (模板已留占位, 注意**不要提交密码**):

```properties
ANDROID_KEYSTORE=D:/path/to/release.jks
ANDROID_KEY_ALIAS=my-key
ANDROID_KEY_PASSWORD=YOUR_PASSWORD
ANDROID_STORE_PASSWORD=YOUR_PASSWORD
```

首次生成 keystore:

```powershell
keytool -genkeypair -v -keystore release.jks -alias my-key `
    -keyalg RSA -keysize 2048 -validity 10000
```

配置好后直接: `.\build.bat --release --run`.

### 模拟器 (AVD) 与 ABI

脚本自动选择 ABI, 无需手动指定:

| 构建方式 | 目标 ABI | Rust target |
|----------|---------|-------------|
| 不带 `--simulator` (真机) | `arm64-v8a` | `aarch64-linux-android` |
| 带 `--simulator <avd>` (模拟器) | `x86_64` | `x86_64-linux-android` |

> ARM 主机 (Apple Silicon / ARM Windows) 上的模拟器用 `arm64-v8a`.
> `abiFilters` 两种 ABI 都列出, 但**只有实际编译出的 `.so` 会被打进 APK**.

| 运行目标 | 是否可用 |
|----------|----------|
| arm64 真机 | ✅ 默认 (不带 `--simulator`) |
| x86_64 模拟器 (Windows / Intel Mac) | ✅ `--simulator <avd>` |
| arm64 模拟器 (ARM 主机) | ✅ `--simulator <avd>` |
| arm64 模拟器 (x86_64 主机) | ❌ 启动直接 FATAL, 见下 |
| x86_64 APK 装进 arm64 真机 | ❌ `INSTALL_FAILED_NO_MATCHING_ABIS` |

arm64 镜像在 x86_64 主机上**无法启动**, 即使 AVD 创建成功也会立刻退出:

```
FATAL | Avd's CPU Architecture 'arm64' is not supported by the QEMU2
      | emulator on x86_64 host. System image must match the host architecture.
```

脚本会在编译前读取 AVD 的 `config.ini`, 若其中的 `abi.type` 与本次自动选定的 ABI
不一致, 会直接报错退出, 不会白等编译.

`--simulator` 只使用**已存在**的 AVD, 不会自动创建. 用新版 Android CLI 管理模拟器:

```powershell
# 查看已有 AVD
& "$env:ANDROID_HOME\emulator\emulator.exe" -list-avds

# 创建模拟器
& "$env:ANDROID_HOME\cmdline-tools\latest\bin\android" emulator create medium_phone

# 启动模拟器
& "$env:ANDROID_HOME\cmdline-tools\latest\bin\android" emulator start medium_phone
```

创建得到的 AVD 架构须与主机一致 (x86_64 主机 → x86_64, ARM 主机 → arm64-v8a),
否则无法启动.

### 日志

`--run` / `--simulator` 在启动后会**自动清空 logcat 并流式捕获日志** (Ctrl+C 结束),
过滤 tag 为本项目 crate 名, 同时保留 `AndroidRuntime:E`(Java 崩溃) 与 `DEBUG:E`(native 崩溃).

手动抓取:

```bash
adb logcat -c
adb logcat -v time -s {{crate_name}}:V AndroidRuntime:E DEBUG:E
```

只保存到文件: `adb logcat -v time -s {{crate_name}}:V > log.txt`

## 注意

- gpui-kit 移动端处于 showcase 阶段, 生产使用前请自测.
- Android 包名由 crate 名派生为 `com.example.{{crate_name}}`; 如需自定义, 修改
  `android/gradle/app/build.gradle.kts` 的 `namespace` / `applicationId`, 以及
  `android/build.bat` / `build.sh` 中的 `am start` 组件名.
- 当前模板仅支持 Android; iOS 支持需引入 XcodeGen + iOS 项目 (参考 gpui-mobile 仓库 example).
- gradle-wrapper.jar 来自 gpui-mobile example, 如需更新运行 `gradle wrapper` 即可.
