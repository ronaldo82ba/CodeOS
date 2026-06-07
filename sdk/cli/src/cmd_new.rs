use std::fs;
use std::path::Path;

pub fn run(name: &str, template: &str) {
    let dest = Path::new(name);
    if dest.exists() {
        eprintln!("error: destination already exists: {name}");
        std::process::exit(1);
    }

    fs::create_dir_all(dest.join("bin")).expect("create dirs");
    fs::create_dir_all(dest.join("assets")).expect("create assets dir");
    fs::create_dir_all(dest.join("res")).expect("create res dir");

    let manifest = format!(
        r#"[app]
id = "com.example.{name}"
name = "{name}"
version = "1.0.0"
min_os_version = "0.1.0"

[entry]
binary = "bin/{name}"
args = []

[ui]
main_view = "res/main_layout.json"
icon = "assets/icon.png"

[permissions]
network = false
storage = true
notifications = false

[metadata]
author = "Ronaldo Mijares"
website = "https://example.com"
description = "Short description of the app."
"#
    );
    fs::write(dest.join("codeos_manifest.toml"), manifest).expect("write manifest");

    let main_rs = if template == "rust-app" || template == "basic-app" {
        format!(
            r#"fn main() {{
    println!("Hello from {{}}!", "{name}");
}}
"#
        )
    } else {
        "fn main() {\n    println!(\"Hello from CodeOS!\");\n}\n".into()
    };
    fs::write(dest.join(format!("bin/{name}")), main_rs).expect("write entry binary");
    fs::write(
        dest.join("res/main_layout.json"),
        r#"{"type":"view","children":[]}"#,
    )
    .expect("write layout");
    fs::write(dest.join("assets/icon.png"), b"PNG").expect("write icon stub");

    println!("Created CodeOS app '{name}' (template: {template})");
    println!("Next: cd {name} && codeos build");
}
