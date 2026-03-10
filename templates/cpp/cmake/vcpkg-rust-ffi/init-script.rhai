let build_type = variable::prompt("选择构建类型", "可执行文件", ["可执行文件", "库文件"]);

if build_type == "可执行文件" {
    variable::set("build_type", "bin");
    variable::set("build_bin", true);
} else {
    variable::set("build_type", "lib");
    variable::set("build_lib", true);
}

if build_type == "库文件" {
    let library_type = variable::prompt("选择构建的库类型", "静态库", ["静态库", "动态库"]);
    if library_type == "动态库" {
        variable::set("library_type", "SHARED");
    } else {
        variable::set("library_type", "STATIC");
    }
}

let os_arch = variable::get("os_arch");
let devcontainer = if os_arch == "linux-x86_64" { true } else { false };
variable::set("devcontainer", devcontainer);

let github_workflow = if os_arch == "windows-x86_64" { true } else { false };
variable::set("github_workflow", github_workflow);
