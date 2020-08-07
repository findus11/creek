use fnv::FnvHashMap;
use std::collections::VecDeque;

use super::problem::{Backward, Forward, Problem};
use super::{Fact, Graph, Node, NodeInfo};

pub struct Analyzer<F, N, G, Trans, Join, Sort>
where
    F: Fact,
    N: Node,
    G: Graph<N>,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut(Vec<F>) -> F,
{
    /// Initial fact for the starting node of the analyzer. In a forwards
    /// problem, this corresponds to the initial `before` fact for the entry
    /// node
    first_fact: NodeInfo<F>,

    /// Initial fact for all non-entering nodes
    init_fact: NodeInfo<F>,

    /// Transition function which computes facts from another fact and its node
    trans: Trans,

    /// Join function which joins multiple facts
    join: Join,

    infos: FnvHashMap<N::NodeId, NodeInfo<F>>,

    _graph: std::marker::PhantomData<G>,
    _node: std::marker::PhantomData<N>,
    _sort: std::marker::PhantomData<Sort>,
}

impl<F, N, G, Trans, Join> Analyzer<F, N, G, Trans, Join, Forward>
where
    F: Fact,
    N: Node,
    G: Graph<N>,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut(Vec<F>) -> F,
{
    /// Create a new forwards problem with the given entry fact, init fact,
    /// transformation function and join function. `top` should have the
    /// property that `join(vec![f, top]) == f` for all facts `f`
    pub fn new_forward(top: F, trans: Trans, join: Join) -> Self {
        Self {
            first_fact: NodeInfo {
                before: top.clone(),
                after: top.clone(),
            },
            init_fact: NodeInfo {
                before: top.clone(),
                after: top,
            },
            trans,
            join,

            infos: FnvHashMap::default(),

            _graph: std::marker::PhantomData,
            _node: std::marker::PhantomData,
            _sort: std::marker::PhantomData,
        }
    }

    pub fn with_entry_fact(self, enter: F) -> Self {
        Self {
            first_fact: NodeInfo {
                before: enter,
                after: self.first_fact.after,
            },
            ..self
        }
    }
}

impl<F, N, G, Trans, Join> Analyzer<F, N, G, Trans, Join, Backward>
where
    F: Fact,
    N: Node,
    G: Graph<N>,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut(Vec<F>) -> F,
{
    /// Create a new backwards problem with the given exit fact, top fact,
    /// transformation function and join function. `top` should have the
    /// property that `join(vec![f, top]) == f` for all facts `f`
    pub fn new_backward(top: F, trans: Trans, join: Join) -> Self {
        Self {
            first_fact: NodeInfo {
                before: top.clone(),
                after: top.clone(),
            },
            init_fact: NodeInfo {
                before: top.clone(),
                after: top,
            },
            trans,
            join,

            infos: FnvHashMap::default(),

            _graph: std::marker::PhantomData,
            _node: std::marker::PhantomData,
            _sort: std::marker::PhantomData,
        }
    }

    pub fn with_exit_fact(self, exit: F) -> Self {
        Self {
            first_fact: NodeInfo {
                before: self.first_fact.before,
                after: exit,
            },
            ..self
        }
    }
}

impl<F, N, G, Trans, Join, Sort> Analyzer<F, N, G, Trans, Join, Sort>
where
    F: Fact,
    N: Node,
    G: Graph<N>,
    Trans: FnMut(&N, F) -> F,
    Join: FnMut(Vec<F>) -> F,
    Sort: Problem<F, N, G>,
{
    pub fn solve(&mut self, graph: &G) -> FnvHashMap<N::NodeId, NodeInfo<F>> {
        // Initialize info map
        self.infos.clear();
        let first = Sort::get_first(graph);
        self.infos.insert(first, self.first_fact.clone());

        // Initialize worklist
        let mut worklist = VecDeque::new();
        worklist.push_back(first);

        while let Some(id) = worklist.pop_front() {
            let node = graph.get(id);

            // Solve new info
            let joined = self.solve_joins(graph, id);
            let transd = (&mut self.trans)(node, joined.clone());

            // Get previous info
            let init_fact = self.init_fact.clone();
            let info = self.infos.entry(id).or_insert(init_fact);
            let prev_trans = Sort::get_join_fact(info);

            if prev_trans != &transd {
                for dirty in Sort::get_nexts(graph, id) {
                    if !worklist.contains(dirty) {
                        worklist.push_back(*dirty);
                    }
                }
            }

            Sort::assign(info, joined, transd);
        }

        self.infos.drain().collect()
    }

    /// Solve the joins for a node
    fn solve_joins(&mut self, graph: &G, node_id: N::NodeId) -> F {
        let mut infos = Vec::new();

        for next in Sort::get_joins(graph, node_id) {
            let init_fact = self.init_fact.clone();
            let next_info = self.infos.entry(*next).or_insert(init_fact);
            infos.push(Sort::get_join_fact(next_info).clone());
        }

        (&mut self.join)(infos)
    }
}
