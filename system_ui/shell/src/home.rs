use codeos_ui::{SceneGraph, SceneGraphBuilder};

pub fn build_home_scene() -> SceneGraph {
    let (builder, _) = SceneGraphBuilder::new().text("Welcome to CodeOS");
    builder.build()
}
