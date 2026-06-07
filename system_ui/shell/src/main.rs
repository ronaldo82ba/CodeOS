mod home;
mod status_bar;

use codeos_ui::{CodeOsTheme, SceneGraphBuilder};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    let theme = CodeOsTheme::dark();
    let (_builder, _text) = SceneGraphBuilder::new().text("CodeOS Home");
    let scene = home::build_home_scene();
    status_bar::render(&theme);

    info!(nodes = scene.node_count(), "CodeUI shell started");
    println!("[CodeUI] Shell running — home, status bar, app switcher (v0.1 stub)");

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c");
    info!("CodeUI shell shutting down");
}
