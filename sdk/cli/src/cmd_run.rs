use std::path::Path;

use codeos_runtime::{AppConfig, AppLoader, AppRuntime, SandboxLevel};

pub fn run(path: &str, simulator: bool) {
    let root = Path::new(path);
    let app = match AppLoader::load(root) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: failed to load app: {e}");
            std::process::exit(1);
        }
    };

    let config = AppConfig {
        app_id: app.manifest.app.id.clone(),
        sandbox: SandboxLevel::Standard,
    };
    let runtime = AppRuntime::new(app, config);

    if simulator {
        println!("[CodeSim] Launching {} in simulator...", runtime.app_id());
    } else {
        println!("[CodeOS] Launching {} on device...", runtime.app_id());
    }

    let cmd = runtime.launch_command();
    println!("Launch command: {}", cmd.join(" "));
}
