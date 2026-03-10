
let project_name = variable::get("project-name");
let project_version = variable::get("project_version");

system::command("vcpkg", ["new", "--application"]);
