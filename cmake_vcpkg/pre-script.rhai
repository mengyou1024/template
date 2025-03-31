let advanced = variable::get("advanced");

if !advanced {
    variable::set("enable_clang_format", true);
    variable::set("enable_clang_tidy", true);
    variable::set("enable_test", true);
}
