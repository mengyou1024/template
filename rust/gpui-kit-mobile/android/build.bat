@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

REM ============================================================================
REM build.bat -- build {{ project-name }} for Android
REM
REM Usage:
REM   build.bat [--release] [--clean] [--run] [--simulator <avd_name>]
REM   默认只构建 APK; --run 安装并启动; --simulator 隐含 --run
REM   ABI 自动选择: 真机 -> arm64-v8a, 模拟器 -> x86_64
REM ============================================================================

set "SCRIPT_DIR=%~dp0"
pushd "%SCRIPT_DIR%.."
set "PROJECT_ROOT=%cd%"
popd

set "ANDROID_GRADLE_DIR=%PROJECT_ROOT%\android\gradle"
set "RUST_SRC_DIR=%PROJECT_ROOT%"

set "PROFILE=debug"
set "CLEAN=false"
set "RUN=false"
set "SIM_AVD="

:parse_args
if "%~1"=="" goto validate
if /I "%~1"=="--release"   set "PROFILE=release"& shift & goto parse_args
if /I "%~1"=="--clean"     set "CLEAN=true"& shift & goto parse_args
if /I "%~1"=="--run"       set "RUN=true"& shift & goto parse_args
if /I "%~1"=="--simulator" set "SIM_AVD=%~2"& set "RUN=true"& shift& shift & goto parse_args
if /I "%~1"=="-h" goto usage
if /I "%~1"=="--help" goto usage
shift
goto parse_args

:validate

REM -- prerequisites ----------------------------------------------------------
where cargo-ndk >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] cargo-ndk not found. Install: cargo install cargo-ndk
    exit /b 1
)

if "%ANDROID_HOME%"=="" if "%ANDROID_SDK_ROOT%"=="" (
    if EXIST "%LOCALAPPDATA%\Android\Sdk" set "ANDROID_HOME=%LOCALAPPDATA%\Android\Sdk"
)
if "%ANDROID_HOME%"=="" (
    echo [WARN] ANDROID_HOME is not set.
)

REM -- resolve ABI (自动) ----------------------------------------------------
REM 真机 -> arm64-v8a; 模拟器 -> x86_64 (ARM 主机上的模拟器用 arm64-v8a)
if "%SIM_AVD%"=="" (
    set "TARGET_ABIS=arm64-v8a"
) else (
    if /I "%PROCESSOR_ARCHITECTURE%"=="ARM64" (
        set "TARGET_ABIS=arm64-v8a"
    ) else (
        set "TARGET_ABIS=x86_64"
    )
)

REM -- AVD 校验 (提前到编译前, 避免白等几十分钟) ------------------------------
if "%SIM_AVD%"=="" goto skip_avd_check

where emulator >nul 2>nul
if %ERRORLEVEL% NEQ 0 if EXIST "%ANDROID_HOME%\emulator\emulator.exe" set "PATH=%ANDROID_HOME%\emulator;!PATH!"

where emulator >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] emulator not found. Is %ANDROID_HOME%\emulator in PATH?
    exit /b 1
)

set "AVD_FOUND="
for /f "usebackq delims=" %%a in (`emulator -list-avds 2^>nul`) do (
    if "%%a"=="%SIM_AVD%" set "AVD_FOUND=1"
)
if not defined AVD_FOUND (
    echo [ERROR] AVD "%SIM_AVD%" not found. Available AVDs:
    emulator -list-avds
    echo [HINT] Create one first:
    echo        android emulator create my_avd
    exit /b 1
)

REM -- AVD 架构校验 (x86_64 主机无法启动 arm64 镜像) -------------------------
set "AVD_ABI="
set "AVD_CFG=%USERPROFILE%\.android\avd\%SIM_AVD%.avd\config.ini"
if not exist "%AVD_CFG%" if defined ANDROID_AVD_HOME set "AVD_CFG=%ANDROID_AVD_HOME%\%SIM_AVD%.avd\config.ini"
if exist "%AVD_CFG%" for /f "usebackq tokens=2 delims==" %%a in (`findstr /b /c:"abi.type=" "%AVD_CFG%"`) do set "AVD_ABI=%%a"

if defined AVD_ABI (
    set "AVD_ABI_OK="
    for %%a in (%TARGET_ABIS%) do if "%%a"=="!AVD_ABI!" set "AVD_ABI_OK=1"
    if not defined AVD_ABI_OK (
        echo [ERROR] AVD "%SIM_AVD%" is !AVD_ABI!, but this build targets %TARGET_ABIS%
        echo         Pick a matching AVD: real device needs arm64-v8a, emulator needs x86_64.
        exit /b 1
    )
)
:skip_avd_check

REM -- map ABI -> rust target / cargo-ndk flags -------------------------------
set "RUST_TARGETS="
set "NDK_T_FLAGS="
for %%a in (%TARGET_ABIS%) do (
    if "%%a"=="arm64-v8a" (
        set "RUST_TARGETS=!RUST_TARGETS! aarch64-linux-android"
        set "NDK_T_FLAGS=!NDK_T_FLAGS! -t arm64-v8a"
    )
    if "%%a"=="x86_64" (
        set "RUST_TARGETS=!RUST_TARGETS! x86_64-linux-android"
        set "NDK_T_FLAGS=!NDK_T_FLAGS! -t x86_64"
    )
)
set "CARGO_PROFILE_FLAG="
set "GRADLE_TASK=assembleDebug"
set "APK_VARIANT=debug"

if /I "%PROFILE%"=="release" (
    set "CARGO_PROFILE_FLAG=--release"
    set "GRADLE_TASK=assembleRelease"
    set "APK_VARIANT=release"
)

REM -- ensure Rust targets ---------------------------------------------------
echo [INFO] Ensuring Rust targets:%RUST_TARGETS%
rustup target add %RUST_TARGETS% >nul 2>nul

REM -- clean (optional) -------------------------------------------------------
if /I "%CLEAN%" NEQ "true" goto skip_clean
echo [INFO] Cleaning...
pushd "%RUST_SRC_DIR%"
for %%t in (%RUST_TARGETS%) do cargo clean --target %%t >nul 2>nul
popd
pushd "%ANDROID_GRADLE_DIR%"
call gradlew.bat clean >nul 2>nul
popd
:skip_clean

REM -- Rust shared library ---------------------------------------------------
echo.
echo [STEP] Building Rust library for %TARGET_ABIS% (%PROFILE%)
set "JNI_LIBS_DIR=%ANDROID_GRADLE_DIR%\app\src\main\jniLibs"

pushd "%RUST_SRC_DIR%"
cargo ndk %NDK_T_FLAGS% -o "%JNI_LIBS_DIR%" --platform 31 build %CARGO_PROFILE_FLAG%
set "RUST_RES=%ERRORLEVEL%"
popd

if %RUST_RES% NEQ 0 (
    echo [ERROR] Rust build failed.
    exit /b 1
)

REM -- assemble APK ----------------------------------------------------------
echo.
echo [STEP] Assembling APK (%GRADLE_TASK%)
pushd "%ANDROID_GRADLE_DIR%"
call gradlew.bat %GRADLE_TASK%
set "GRADLE_RES=%ERRORLEVEL%"
popd

if %GRADLE_RES% NEQ 0 (
    echo [ERROR] Gradle build failed.
    exit /b 1
)

set "APK_PATH=%ANDROID_GRADLE_DIR%\app\build\outputs\apk\%APK_VARIANT%\app-%APK_VARIANT%.apk"
if NOT EXIST "%APK_PATH%" (
    echo [ERROR] APK not found: %APK_PATH%
    exit /b 1
)
echo [INFO] APK: %APK_PATH%

REM -- skip run --------------------------------------------------------------
if /I not "%RUN%"=="true" (
    echo [INFO] Skipping install ^& launch ^(use --run to install ^& launch^).
    goto done
)

REM -- locate adb ------------------------------------------------------------
where adb >nul 2>nul
if %ERRORLEVEL% NEQ 0 if EXIST "%ANDROID_HOME%\platform-tools\adb.exe" set "PATH=%ANDROID_HOME%\platform-tools;!PATH!"

where adb >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] adb not found. Add platform-tools to PATH.
    exit /b 1
)

REM -- start emulator (if requested) ----------------------------------------
if "%SIM_AVD%"=="" goto skip_simulator

echo.
echo [STEP] Starting emulator: %SIM_AVD%
start "" /B emulator -avd %SIM_AVD% -no-snapshot-load -no-audio -gpu swiftshader_indirect >nul 2>nul

echo [INFO] Waiting for emulator to boot...
adb wait-for-device >nul 2>nul

set "BOOTED=0"
for /L %%i in (1,1,60) do (
    if !BOOTED! EQU 0 (
        for /f "usebackq delims=" %%v in (`adb shell getprop sys.boot_completed 2^>nul`) do (
            if "%%v"=="1" set "BOOTED=1"
        )
        if !BOOTED! EQU 0 (
            timeout /t 2 /nobreak >nul
        )
    )
)
if !BOOTED! EQU 0 (
    echo [WARN] Emulator did not finish booting in 120s, continuing anyway...
) else (
    echo [INFO] Emulator ready.
)
:skip_simulator

REM -- device check ----------------------------------------------------------
echo.
echo [STEP] Installing ^& launching

adb devices | findstr /v "List" | findstr "device" >nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] No connected Android device/emulator.
    exit /b 1
)

adb logcat -c >nul 2>nul

adb install -r "%APK_PATH%"
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] APK install failed.
    exit /b 1
)

adb shell am start -n "com.example.{{crate_name}}/android.app.NativeActivity" -a android.intent.action.MAIN -c android.intent.category.LAUNCHER

echo.
echo [INFO] Launched. Streaming logcat ^(tag: {{crate_name}}, Ctrl+C to stop^)...
echo.

adb logcat -v time -s "{{crate_name}}:V" "AndroidRuntime:E" "DEBUG:E"

:done
echo.
echo [INFO] Done!
exit /b 0

:usage
echo Usage: build.bat [--release] [--clean] [--run] [--simulator ^<avd_name^>]
echo.
echo   --release       Build release (smaller, slower to compile)
echo   --clean         Clean target directories before building
echo   --run           Install ^& launch on a connected device
echo   --simulator ^<name^>  Start the given AVD emulator, then install ^& launch ^(implies --run^)
echo.
echo Default builds the APK only. ABI is selected automatically:
echo real device -^> arm64-v8a, emulator -^> x86_64.
echo.
echo Examples:
echo   build.bat
echo   build.bat --release
echo   build.bat --run
echo   build.bat --simulator Pixel_8_API_34
exit /b 0
