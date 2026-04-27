use cmake_preset::*;

fn main() {
    if cfg!(target_os = "windows") {
        CMakePresetBuilder::new()
            .set_project_dir("cpp-lib")
            .set_config_preset("msvc-x64-static-mt-rel")
            .set_library_name("cpp-lib")
            .config()
            .build();
    } else if cfg!(target_os = "linux") {
        CMakePresetBuilder::new()
            .set_project_dir("cpp-lib")
            .set_config_preset("gcc-x64-static-rel")
            .set_library_name("cpp-lib")
            .config()
            .build();
        // Link libstdc++
        println!("cargo:rustc-link-lib=static=stdc++");
    }
}
