let project_name_upper = variable::get("project-name").to_shouty_snake_case();
variable::set("project_name_upper", project_name_upper);

let advanced = variable::get("advanced");

if !advanced {
    variable::set("enable_clang_format", true);
    variable::set("enable_clang_tidy", true);
    variable::set("enable_test", true);
}
