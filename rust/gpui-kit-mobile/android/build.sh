#!/usr/bin/env bash
# build.sh -- Build & run {{ project-name }} on Android
#
# Usage:
#   ./build.sh [--release] [--clean] [--run] [--simulator <avd_name>]
#
# 默认只构建 APK; --run 安装并启动; --simulator 隐含 --run
# ABI 自动选择: 真机 -> arm64-v8a, 模拟器 -> x86_64
#
# Prerequisites:
#   Android SDK + NDK, cargo-ndk (cargo install cargo-ndk)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ANDROID_GRADLE_DIR="$SCRIPT_DIR/gradle"

PROFILE="debug"
CLEAN=false
RUN=false
SIM_AVD=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --release)   PROFILE="release" ;;
        --clean)     CLEAN=true ;;
        --run)       RUN=true ;;
        --simulator) SIM_AVD="$2"; RUN=true; shift ;;
        -h|--help)   cat <<EOF
Usage: ./build.sh [--release] [--clean] [--run] [--simulator <avd_name>]

  --release       Build release (smaller, slower to compile)
  --clean         Clean target directories before building
  --run           Install & launch on a connected device
  --simulator <n> Start the given AVD emulator, then install & launch (implies --run)

Default builds the APK only. ABI is selected automatically:
real device -> arm64-v8a, emulator -> x86_64.

Examples:
  ./build.sh
  ./build.sh --release
  ./build.sh --run
  ./build.sh --simulator Pixel_8_API_34
EOF
            exit 0 ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
    shift
done

if ! command -v cargo-ndk &>/dev/null; then
    echo "[ERROR] cargo-ndk not found. Install: cargo install cargo-ndk"
    exit 1
fi

# -- resolve ABI (自动) ------------------------------------------------------
# 真机 -> arm64-v8a; 模拟器 -> x86_64 (ARM 主机上的模拟器用 arm64-v8a)
HOST_ARCH="$(uname -m)"
if [[ -z "$SIM_AVD" ]]; then
    ABIS="arm64-v8a"
elif [[ "$HOST_ARCH" == "arm64" || "$HOST_ARCH" == "aarch64" ]]; then
    ABIS="arm64-v8a"
else
    ABIS="x86_64"
fi

# -- AVD 校验 (提前到编译前, 避免白等) ---------------------------------------
if [[ -n "$SIM_AVD" ]]; then
    if ! command -v emulator &>/dev/null; then
        if [[ -n "${ANDROID_HOME:-}" && -x "$ANDROID_HOME/emulator/emulator" ]]; then
            export PATH="$ANDROID_HOME/emulator:$PATH"
        fi
    fi
    if ! command -v emulator &>/dev/null; then
        echo "[ERROR] emulator not found. Is \$ANDROID_HOME/emulator in PATH?"
        exit 1
    fi
    if ! emulator -list-avds 2>/dev/null | tr -d '\r' | grep -qx "$SIM_AVD"; then
        echo "[ERROR] AVD '$SIM_AVD' not found. Available AVDs:"
        emulator -list-avds 2>/dev/null || true
        echo "[HINT] Create one first:"
        echo "       android emulator create my_avd"
        exit 1
    fi

    # AVD 架构校验 (x86_64 主机无法启动 arm64 镜像)
    AVD_CFG="${ANDROID_AVD_HOME:-$HOME/.android/avd}/$SIM_AVD.avd/config.ini"
    AVD_ABI=""
    if [[ -f "$AVD_CFG" ]]; then
        AVD_ABI="$(awk -F= '/^abi\.type=/{print $2}' "$AVD_CFG" | tr -d '\r')"
    fi
    if [[ -n "$AVD_ABI" ]]; then
        ABI_OK=false
        for a in $ABIS; do
            if [[ "$a" == "$AVD_ABI" ]]; then ABI_OK=true; fi
        done
        if ! $ABI_OK; then
            echo "[ERROR] AVD '$SIM_AVD' is $AVD_ABI, but this build targets $ABIS"
            echo "        Pick a matching AVD: real device needs arm64-v8a, emulator needs x86_64."
            exit 1
        fi
    fi
fi

# -- map ABI -> rust target / cargo-ndk flags -------------------------------
RUST_TARGETS=()
NDK_T_FLAGS=()
for a in $ABIS; do
    case "$a" in
        arm64-v8a)
            RUST_TARGETS+=("aarch64-linux-android")
            NDK_T_FLAGS+=("-t" "arm64-v8a") ;;
        x86_64)
            RUST_TARGETS+=("x86_64-linux-android")
            NDK_T_FLAGS+=("-t" "x86_64") ;;
    esac
done

CARGO_PROFILE_FLAG=""
GRADLE_TASK="assembleDebug"
APK_VARIANT="debug"

if [[ "$PROFILE" == "release" ]]; then
    CARGO_PROFILE_FLAG="--release"
    GRADLE_TASK="assembleRelease"
    APK_VARIANT="release"
fi

echo "[INFO] Ensuring Rust targets: ${RUST_TARGETS[*]}"
rustup target add "${RUST_TARGETS[@]}" 2>/dev/null || true

if $CLEAN; then
    echo "[INFO] Cleaning..."
    cd "$PROJECT_ROOT"
    for t in "${RUST_TARGETS[@]}"; do
        cargo clean --target "$t" 2>/dev/null || true
    done
    cd "$ANDROID_GRADLE_DIR"
    ./gradlew clean 2>/dev/null || true
fi

echo ""
echo "[STEP] Building Rust library for $ABIS ($PROFILE)"
JNI_LIBS_DIR="$ANDROID_GRADLE_DIR/app/src/main/jniLibs"

cd "$PROJECT_ROOT"
cargo ndk "${NDK_T_FLAGS[@]}" -o "$JNI_LIBS_DIR" --platform 31 build $CARGO_PROFILE_FLAG

echo ""
echo "[STEP] Assembling APK ($GRADLE_TASK)"
cd "$ANDROID_GRADLE_DIR"
./gradlew "$GRADLE_TASK"

APK_PATH="$ANDROID_GRADLE_DIR/app/build/outputs/apk/$APK_VARIANT/app-$APK_VARIANT.apk"
echo "[INFO] APK: $APK_PATH"

if ! $RUN; then
    echo "[INFO] Skipping install & launch (use --run to install & launch)."
    exit 0
fi

# locate adb
if ! command -v adb &>/dev/null; then
    if [[ -n "${ANDROID_HOME:-}" && -x "$ANDROID_HOME/platform-tools/adb" ]]; then
        export PATH="$ANDROID_HOME/platform-tools:$PATH"
    fi
fi
if ! command -v adb &>/dev/null; then
    echo "[ERROR] adb not found. Add platform-tools to PATH."
    exit 1
fi

# start emulator if requested
if [[ -n "$SIM_AVD" ]]; then
    echo ""
    echo "[STEP] Starting emulator: $SIM_AVD"
    emulator -avd "$SIM_AVD" -no-snapshot-load -no-audio -gpu swiftshader_indirect &>/dev/null &

    echo "[INFO] Waiting for emulator to boot..."
    adb wait-for-device

    BOOTED=0
    for _ in $(seq 1 60); do
        if [[ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" == "1" ]]; then
            BOOTED=1
            break
        fi
        sleep 2
    done
    if [[ $BOOTED -eq 0 ]]; then
        echo "[WARN] Emulator did not finish booting in 120s, continuing anyway..."
    else
        echo "[INFO] Emulator ready."
    fi
fi

echo ""
echo "[STEP] Installing & launching"

if ! adb devices | tail -n +2 | grep -q "device$"; then
    echo "[ERROR] No connected Android device/emulator."
    exit 1
fi

adb logcat -c 2>/dev/null || true

adb install -r "$APK_PATH" || { echo "[ERROR] APK install failed."; exit 1; }

adb shell am start \
    -n "com.example.{{crate_name}}/android.app.NativeActivity" \
    -a android.intent.action.MAIN \
    -c android.intent.category.LAUNCHER

echo ""
echo "[INFO] Launched. Streaming logcat (tag: {{crate_name}}, Ctrl+C to stop)..."
echo ""

adb logcat -v time -s "{{crate_name}}:V" "AndroidRuntime:E" "DEBUG:E"
