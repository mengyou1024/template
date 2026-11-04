let os_arch = variable::get("os-arch");

let uuid = if os_arch.starts_with("windows-") {
    system::command(
        "powershell",
        ["-NoProfile", "-Command", "[System.Guid]::NewGuid().ToString()"],
    )
} else if os_arch.starts_with("linux-") {
    system::command("cat", ["/proc/sys/kernel/random/uuid"])
} else {
    system::command("uuidgen")
};

variable::set("app_instance_id", "{" + uuid.to_upper() + "}");
