use core::{edge::Edge, graph::Graph, vertex::Vertex};

mod core;

fn main() {
    let mut graph = Graph::new();

    let belem = Vertex::new("Belém");
    let barcarena = Vertex::new("Barcarena");
    let ananindeua = Vertex::new("Ananindeua");

    graph.add_vertex(belem.clone());
    graph.add_vertex(barcarena.clone());
    graph.add_vertex(ananindeua.clone());

    graph.add_edge(Edge::new(&barcarena, &belem, 10));
    graph.add_edge(Edge::new(&ananindeua, &belem, 15));
    graph.add_edge(Edge::new(&barcarena, &ananindeua, 20));

    let max_flow = graph.ford_fulkerson("Barcarena", "Belém");

    println!("Fluxo máximo de entregas: {}", max_flow);
}
