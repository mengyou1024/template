use cmake_preset::*;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("Cargo did not provide CARGO_CFG_TARGET_ARCH");

    let target_os =
        std::env::var("CARGO_CFG_TARGET_OS").expect("Cargo did not provide CARGO_CFG_TARGET_OS");

    if target_arch != "x86_64" {
        panic!("Unsupported target architecture: {target_arch}");
    }

    if target_os == "windows" {
        CMakePresetBuilder::new()
            .set_project_dir("cpp-lib")
            .set_config_preset("msvc-x64-mt-static-rel")
            .set_library_name("cpp-lib")
            .config()
            .build();
    } else if target_os == "linux" {
        CMakePresetBuilder::new()
            .set_project_dir("cpp-lib")
            .set_config_preset("gcc-x64-static-rel")
            .set_library_name("cpp-lib")
            .config()
            .build();
        // Link libstdc++
        println!("cargo:rustc-link-lib=static=stdc++");
    } else {
        panic!("Unsupported target_os: {target_os}");
    }
}
