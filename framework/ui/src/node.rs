use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeKind {
    Column,
    Row,
    Text { content: String },
    Image { source: String },
    Spacer,
    Surface,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneNode {
    pub id: NodeId,
    pub kind: NodeKind,
    #[serde(default)]
    pub children: Vec<NodeId>,
    pub layout: LayoutProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutProps {
    pub flex: f32,
    pub padding: u32,
}

impl Default for LayoutProps {
    fn default() -> Self {
        Self { flex: 1.0, padding: 0 }
    }
}
