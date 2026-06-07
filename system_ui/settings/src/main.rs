mod store;

use codeos_ui::SceneGraphBuilder;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    let store = store::SettingsStore::default();
    let (builder, _) = SceneGraphBuilder::new().text("Settings");
    let scene = builder.build();

    info!(entries = store.len(), nodes = scene.node_count(), "CodeUI settings started");
    println!("[CodeUI] Settings app (v0.1 stub)");
}
