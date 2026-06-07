#[derive(Debug, Clone, Copy)]
pub struct LayoutConstraints {
    pub max_width: u32,
    pub max_height: u32,
}

#[derive(Debug, Clone)]
pub struct FlowLayout;

impl FlowLayout {
    pub fn measure(nodes: usize, constraints: LayoutConstraints) -> (u32, u32) {
        let row_height = constraints.max_height / nodes.max(1) as u32;
        (constraints.max_width, row_height * nodes as u32)
    }
}
