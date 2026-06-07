use std::collections::HashMap;

use crate::{NodeId, SceneNode};

pub struct SceneGraph {
    nodes: HashMap<NodeId, SceneNode>,
    root: Option<NodeId>,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root: None,
        }
    }

    pub fn insert(&mut self, node: SceneNode) {
        if self.root.is_none() {
            self.root = Some(node.id);
        }
        self.nodes.insert(node.id, node);
    }

    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    pub fn get(&self, id: NodeId) -> Option<&SceneNode> {
        self.nodes.get(&id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SceneGraphBuilder {
    next_id: u64,
    graph: SceneGraph,
}

impl SceneGraphBuilder {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            graph: SceneGraph::new(),
        }
    }

    fn alloc_id(&mut self) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn text(mut self, content: impl Into<String>) -> (Self, NodeId) {
        let id = self.alloc_id();
        self.graph.insert(SceneNode {
            id,
            kind: crate::NodeKind::Text {
                content: content.into(),
            },
            children: vec![],
            layout: Default::default(),
        });
        (self, id)
    }

    pub fn build(self) -> SceneGraph {
        self.graph
    }
}

impl Default for SceneGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}
