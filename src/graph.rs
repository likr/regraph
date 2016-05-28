pub struct Node {
    pub layer: usize,
    pub order: usize,
    pub width: usize,
    pub height: usize,
    pub x: i32,
    pub y: i32,
    pub dummy: bool,
}

impl Node {
    pub fn new() -> Node {
        Node {
            layer: 0,
            order: 0,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            dummy: false,
        }
    }
}

pub struct Edge {
    pub conflict: bool,
}

impl Edge {
    pub fn new() -> Edge {
        Edge {
            conflict: false,
        }
    }
}
