mod analyze;
mod problem;

pub use analyze::Analyzer;
pub use problem::{Backward, Forward};

use std::hash::Hash;

/// A fact represents a piece of information known to be true at a particular
/// point in the graph. In a constant propagation problem, for instance, a fact
/// might be a set of tuples of variables known to be constant and their value
pub trait Fact: Clone + PartialEq {}

/// A graph is a set of nodes, each of which is only connected to nodes in this
/// graph
pub trait Graph<N: Node> {
    /// Get a node with a given `id`. This `id` only comes from `get_entry`,
    /// `get_exit`, or a node returned by `get`'s predecessors or successors.
    fn get(&self, id: N::NodeId) -> &N;

    /// Get all nodes in this graph
    fn get_all(&self) -> Vec<N::NodeId>;

    /// Get the entry node
    fn get_entry(&self) -> N::NodeId;

    /// Get the exit node
    fn get_exit(&self) -> N::NodeId;
}

/// A node in a directed graph can have predecessors, which are other nodes that
/// point to this one, as well as successors, which are nodes that this points
/// to.
///
/// Nodes are uniquely identified by a `NodeId`.
pub trait Node {
    /// A small value used to uniquely implement
    type NodeId: Copy + Eq + Hash;

    /// Get the predecessors for this node
    fn get_preds(&self) -> &[Self::NodeId];

    /// Get the successors for this node
    fn get_succs(&self) -> &[Self::NodeId];
}

/// The information which holds true at the `in` and `out` points of a
/// particular node
#[derive(Clone, Debug, PartialEq)]
pub struct NodeInfo<F: Fact> {
    pub before: F,
    pub after: F,
}
