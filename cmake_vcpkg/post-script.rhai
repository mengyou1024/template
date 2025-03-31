
let project_name = variable::get("project-name");
let project_version = variable::get("project_version");

system::command("git", ["init"]);
system::command("vcpkg", ["new", "--name", project_name, "--version", project_version]);
