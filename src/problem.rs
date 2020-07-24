//use super::{Fact, Node};

//pub(crate) trait Problem {}

pub struct Forward;
//impl Problem for Forward {}

pub struct Backward;
//impl Problem for Backward {}


/*
pub struct ForwardProblem<F, N, Trans, Join>
where
    F: Fact,
    N: Node,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut(Vec<F>) -> F,
{
    /// Initial fact for entry node
    pub(crate) entry: F,
    
    /// Initial fact for all non-entry nodes. For all facts `f`, `join(f, init)` 
    /// should equal `f`
    pub(crate) init: F,

    /// The transition function computes the `after` fact for a node in terms of 
    /// its `before` fact
    pub(crate) trans: Trans,

    /// The join function computes the `before` fact for a node in terms of its
    /// predecessors' `after` facts
    pub(crate) join: Join,

    _n: std::marker::PhantomData<N>,
}

impl<F, N, Trans, Join> ForwardProblem<F, N, Trans, Join>
where 
    F: Fact,
    N: Node,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut(Vec<F>) -> F,
{
    /// Create a new forwards problem. `entry` gives the initial fact true for
    /// the entry node, `init` gives the initial fact for all other nodes, 
    /// `trans` calculates the `after` fact in terms of a node and its `before` 
    /// fact, and `join` calculates the `before` fact in terms of all of its 
    /// predecessors' `after` facts
    pub fn new(entry: F, init: F, trans: Trans, join: Join) -> Self {
        Self {
            entry,
            init,
            trans,
            join,
            _n: std::marker::PhantomData,
        }
    }
}

impl<F, N, Trans, Join> Problem for ForwardProblem<F, N, Trans, Join>
where 
    F: Fact, 
    N: Node,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut(Vec<F>) -> F,
{
}


pub struct BackwardProblem<F, N, Trans, Join>
where
    F: Fact,
    N: Node,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut
*/