use github_actions::{get_input, info, set_output};

fn main() {
    info!("--- Started action ---");

    let message: String = get_input("message").unwrap();

    info!("echo message: {}", message);

    let name = get_input("name").unwrap();

    set_output("greeting", format!("Hello, {}!", name)).unwrap();

    info!("--- Exited Action ---");
}
