mod analyze;
mod problem;

pub use problem::{Backward, Forward};

use std::hash::Hash;

/// A fact represents a piece of information known to be true at a particular
/// point in the graph. In a constant propagation problem, for instance, a fact
/// might be a set of tuples of variables known to be constant and their value
pub trait Fact
where
    Self: Clone + PartialEq
{}

/// A node in a directed graph can have predecessors, which are other nodes that
/// point to this one, as well as successors, which are nodes that this points 
/// to. 
///
/// Nodes are uniquely identified by a `NodeId`.
pub trait Node {
    /// A small value used to uniquely implement 
    type NodeId: Copy + Hash;

    /// Get the predecessors for this node
    fn get_preds(&self) -> &[Self::NodeId];
    
    /// Get the successors for this node
    fn get_succs(&self) -> &[Self::NodeId];
}
