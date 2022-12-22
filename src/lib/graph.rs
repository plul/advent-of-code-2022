use std::borrow::Cow;

pub mod topological_sort;
pub mod dijkstra;

/// Directed graph.
pub trait Graph<'g> {
    type Node: Clone + 'g;
    type Edge: GraphEdge<'g, Node = Self::Node>;

    /// Return edges from a given node.
    fn edges(&'g self, from: &Self::Node) -> Vec<Self::Edge>;
}

pub trait GraphEdge<'g> {
    type Node: Clone;

    /// Return the node that this edge leads to.
    fn to(&self) -> Cow<'g, Self::Node>;
}

pub trait GraphEdgeCost {
    type Cost;

    /// Return cost of edge.
    fn cost(&self) -> Self::Cost;
}
