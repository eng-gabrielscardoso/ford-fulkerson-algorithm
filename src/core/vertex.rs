#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub id: String,
}

impl Vertex {
    pub fn new(id: &str) -> Self {
        Vertex { id: id.to_string() }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}
