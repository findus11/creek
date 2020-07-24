use super::{Fact, Node};
use super::problem::{Backward, Forward};

pub struct Analyzer<F, N, Trans, Join, Sort>
where
    F: Fact,
    N: Node,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut(Vec<F>) -> F,
{
    /// Initial fact for the starting node of the analyzer. In a forwards 
    /// problem, this corresponds to the initial `before` fact for the entry 
    /// node
    enter_fact: F,

    /// Initial fact for all non-entering nodes
    init_fact: F,

    /// Transition function which computes facts from another fact and its node
    trans: Trans,

    /// Join function which joins multiple facts
    join: Join,


    _node: std::marker::PhantomData<N>,
    _sort: std::marker::PhantomData<Sort>,
}

impl<F, N, Trans, Join> for Analyzer<F, N, Trans, Join, Forward>
where 
    F: Fact,
    N: Node,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut(Vec<F>) -> F,
{
    /// Create a new forwards problem 
    pub fn new_forward(enter: F, init: F, trans: Trans, join: Join) -> Self {

    }
}