use super::Graph;
use super::GraphEdge;
use super::GraphEdgeCost;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

/// Dijkstra's algo implemented with a min-heap.
pub fn shortest_path<'g, G>(graph: &'g G, start_node: &'g G::Node, end_node: &'g G::Node) -> Option<<G::Edge as GraphEdgeCost>::Cost>
where
    G: Graph<'g>,
    G::Node: Eq + Hash,
    G::Edge: GraphEdgeCost,
    <G::Edge as GraphEdgeCost>::Cost: Ord + Clone,
    <G::Edge as GraphEdgeCost>::Cost: Add<Output = <G::Edge as GraphEdgeCost>::Cost>,
{
    // Min-heap of edges by CUMULATIVE cost.
    let mut edges: BinaryHeap<Reverse<Edge<'g, G>>> = BinaryHeap::new();

    let i = graph
        .edges(start_node)
        .into_iter()
        .map(|edge| {
            let to = edge.to();
            Edge { edge_cost: edge.cost(), to }
        })
        .map(Reverse);
    edges.extend(i);

    let mut visited_nodes = HashSet::<Cow<'_, G::Node>>::new();
    visited_nodes.insert(Cow::Borrowed(start_node));

    while let Some(Reverse(edge)) = edges.pop() {
        if visited_nodes.contains(&edge.to) {
            continue;
        }

        if edge.to.as_ref() == end_node {
            return Some(edge.edge_cost);
        }

        edges.extend(
            graph
                .edges(&edge.to)
                .into_iter()
                .filter(|e| !visited_nodes.contains(&e.to()))
                .map(|e| {
                    let cumulative_edge_cost = e.cost() + edge.edge_cost.clone();
                    Edge {
                        to: e.to(),
                        edge_cost: cumulative_edge_cost,
                    }
                })
                .map(Reverse),
        );
        visited_nodes.insert(edge.to);
    }

    None
}

/// A graph edge that is ordered solely by the `edge_cost` field.
struct Edge<'a, G>
where
    G: Graph<'a>,
    G::Edge: GraphEdgeCost,
{
    edge_cost: <G::Edge as GraphEdgeCost>::Cost,
    to: Cow<'a, G::Node>,
}

impl<'a, G> PartialEq for Edge<'a, G>
where
    G: Graph<'a>,
    G::Node: PartialEq,
    G::Edge: GraphEdgeCost,
    <G::Edge as GraphEdgeCost>::Cost: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.edge_cost == other.edge_cost && self.to == other.to
    }
}

impl<'a, G> Eq for Edge<'a, G>
where
    G: Graph<'a>,
    G::Node: Eq,
    G::Edge: GraphEdgeCost,
    <G::Edge as GraphEdgeCost>::Cost: Eq,
{
}

/// Only edge cost is considered.
impl<'a, G> PartialOrd for Edge<'a, G>
where
    G: Graph<'a>,
    G::Node: PartialEq,
    G::Edge: GraphEdgeCost,
    <G::Edge as GraphEdgeCost>::Cost: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.edge_cost.partial_cmp(&other.edge_cost)
    }
}

/// Only edge cost is considered.
impl<'a, G> Ord for Edge<'a, G>
where
    G: Graph<'a>,
    G::Node: Eq,
    G::Edge: GraphEdgeCost,
    <G::Edge as GraphEdgeCost>::Cost: Ord + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.edge_cost.cmp(&other.edge_cost)
    }
}
