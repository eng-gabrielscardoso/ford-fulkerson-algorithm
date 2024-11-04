use super::vertex::Vertex;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Edge {
    pub source: Vertex,
    pub target: Vertex,
    pub capacity: i32,
}

impl Edge {
    pub fn new(source: &Vertex, target: &Vertex, capacity: i32) -> Self {
        Edge {
            source: source.clone(),
            target: target.clone(),
            capacity,
        }
    }
}
