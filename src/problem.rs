use super::{Fact, Graph, Node, NodeInfo};

pub trait Problem<F: Fact, N: Node, G: Graph<N>>: private::Sealed {
    /// Assign the `joined` and `transd` facts to a `NodeInfo`
    fn assign(info: &mut NodeInfo<F>, joined: F, transd: F);

    /// Get the `NodeId`s for the nodes that need to be analyzed after this. In
    /// a forwards problem, this corresponds to a node's successors.
    fn get_nexts(node: &N) -> &[N::NodeId];

    /// Get the `NodeId`s for the nodes whose facts are to be joined together.
    /// In a forwards problem, this corresponds to a node's predecessors.
    fn get_joins(node: &N) -> &[N::NodeId];

    /// Get the node id for which the `first` fact holds true. In a forwards
    /// problem, this is the entry node.
    fn get_first(graph: &G) -> N::NodeId;

    /// Get the fact which is computed by transformation. Note that this is not
    /// the fact that will be transformed, but the fact that has been 
    /// transformed. This also gives the fact which will be `join`ed with other
    /// facts.
    fn get_join_fact(info: &NodeInfo<F>) -> &F;
}

/// A forwards problem finds information which holds on all paths from `enter`
/// to `n` for all nodes `n`. Forward problems are often (but not always)
/// problems where all facts *must* be true
pub struct Forward;
impl<F, N, G> Problem<F, N, G> for Forward
where
    F: Fact,
    N: Node,
    G: Graph<N>,
{
    fn assign(info: &mut NodeInfo<F>, joined: F, transd: F) {
        info.before = joined;
        info.after = transd;
    }

    fn get_nexts(node: &N) -> &[N::NodeId] {
        node.get_succs()
    }

    fn get_joins(node: &N) -> &[N::NodeId] {
        node.get_preds()
    }

    fn get_first(graph: &G) -> N::NodeId {
        graph.get_entry()
    }

    fn get_join_fact(info: &NodeInfo<F>) -> &F {
        &info.after
    }
}

/// A backwards problem finds information which holds on all paths from `n` to
/// `exit` for all nodes `n`. Backward problems are often (but not always)
/// problems where facts *may* be true
pub struct Backward;
impl<F, N, G> Problem<F, N, G> for Backward
where
    F: Fact,
    N: Node,
    G: Graph<N>,
{
    fn assign(info: &mut NodeInfo<F>, joined: F, transd: F) {
        info.before = transd;
        info.after = joined;
    }

    fn get_nexts(node: &N) -> &[N::NodeId] {
        node.get_preds()
    }

    fn get_joins(node: &N) -> &[N::NodeId] {
        node.get_succs()
    }

    fn get_first(graph: &G) -> N::NodeId {
        graph.get_exit()
    }

    fn get_join_fact(info: &NodeInfo<F>) -> &F {
        &info.before
    }
}

mod private {
    /// Disallows downstream implementations of `Problem`
    pub trait Sealed {}
    impl Sealed for super::Forward {}
    impl Sealed for super::Backward {}
}
