use std::{
    collections::{HashMap, HashSet},
    i32,
};

use super::{edge::Edge, vertex::Vertex};

#[derive(Debug, Clone)]
pub struct Graph {
    pub edges: HashSet<Edge>,
    pub vertices: HashSet<Vertex>,
    pub residual: HashMap<String, Vec<(String, i32)>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashSet::new(),
            vertices: HashSet::new(),
            residual: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.insert(vertex.clone()); // Clonando aqui
        self.residual
            .insert(vertex.get_id().to_string(), Vec::new());
    }


    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge.clone());
        self.residual
            .entry(edge.source.get_id().to_string())
            .or_default()
            .push((edge.target.get_id().to_string(), edge.capacity));
    }

    pub fn ford_fulkerson(&self, source: &str, sink: &str) -> i32 {
        let mut max_flow = 0;
        let mut residual_graph = self.residual.clone();

        loop {
            let mut visited = HashSet::new();
            let flow = self.dfs(source, sink, i32::MAX, &mut visited, &mut residual_graph);
            if flow == 0 {
                break;
            }
            max_flow += flow;
        }

        max_flow
    }

    fn dfs(
        &self,
        u: &str,
        sink: &str,
        flow: i32,
        visited: &mut HashSet<String>,
        residual_graph: &mut HashMap<String, Vec<(String, i32)>>,
    ) -> i32 {
        if u == sink {
            return flow;
        }

        visited.insert(u.to_string());

        let neighbors = residual_graph.get(u).cloned();

        if let Some(neighbors) = neighbors {
            for (v, capacity) in neighbors {
                if capacity > 0 && !visited.contains(&v) {
                    let current_flow = self.dfs(&v, sink, flow.min(capacity), visited, residual_graph);

                    if current_flow > 0 {
                        if let Some(neighbors_mut) = residual_graph.get_mut(u) {
                            if let Some(pos) = neighbors_mut.iter_mut().position(|(neighbor, _)| neighbor == &v) {
                                let (_, cap) = &mut neighbors_mut[pos];
                                *cap -= current_flow;
                            }
                        }

                        let entry = residual_graph.entry(v).or_insert_with(Vec::new);
                        entry.push((u.to_string(), current_flow));

                        return current_flow;
                    }
                }
            }
        }

        0
    }
}
