use super::Graph;
use super::GraphEdge;
use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::Hash;

/// Implemented with a modified DFS.
///
/// TODO: Implement some form of check that this is really a DAG - what happens if it is not?
pub fn topological_sort<'g, G>(graph: &'g G, start_node: &'g G::Node) -> Vec<Cow<'g, G::Node>>
where
    G: Graph<'g>,
    G::Node: Eq + Hash,
{
    let mut topological_order: Vec<Cow<'g, G::Node>> = vec![];
    let mut visited_nodes = HashSet::<Cow<'_, G::Node>>::new();
    visited_nodes.insert(Cow::Borrowed(start_node));
    let mut stack: Vec<(Cow<'g, G::Node>, Vec<G::Edge>)> =
        vec![(Cow::Borrowed(start_node), graph.edges(start_node))];

    while let Some((_node, node_edges)) = stack.last_mut() {
        if let Some(edge) = node_edges.pop() {
            let adjacent_node = edge.to();
            visited_nodes.insert(adjacent_node.clone());
            let mut adjacent_node_edges = graph.edges(&adjacent_node);
            adjacent_node_edges.retain(|e| !visited_nodes.contains(&e.to()));
            stack.push((adjacent_node, adjacent_node_edges));
        } else {
            let (node, _edges) = stack.pop().unwrap();
            topological_order.push(node);
        }
    }

    topological_order
}
