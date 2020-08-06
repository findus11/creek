//! Liveness analysis finds all variables which may be used after a given point.
//!
//! Liveness analysis is a forwards problem with these functions
//!
//! ```plain
//! trans(b) = union(gen(b), in(b) - kill(b))
//! join = union
//! ```

mod cfg;

use cfg::*;
use creek::{Analyzer, Fact, NodeInfo};
use fnv::FnvHashSet;

#[derive(Clone, Debug, PartialEq)]
struct LivenessFact {
    live: FnvHashSet<Variable>,
}

impl LivenessFact {
    fn new(live: FnvHashSet<Variable>) -> Self {
        Self { live }
    }
}

impl Fact for LivenessFact {}

/// ```plain
/// trans(b) = union(gen(b), in(b) - kill(b))
/// ```
fn trans(block: &Block, fact: LivenessFact) -> LivenessFact {
    let mut used = FnvHashSet::default();
    let mut killed = FnvHashSet::default();

    for stmt in block.stmts.iter() {
        match stmt {
            Statement::ConstAssign(var, _) => {
                killed.insert(*var);
            }
            Statement::VarAssign(var, war) => {
                killed.insert(*var);
                used.insert(*war);
            }
        }
    }

    for var in fact.live {
        if !killed.contains(&var) {
            used.insert(var);
        }
    }

    LivenessFact { live: used }
}

/// ```plain
/// join = union
/// ```
fn join(facts: Vec<LivenessFact>) -> LivenessFact {
    let mut res = FnvHashSet::default();

    for fact in facts {
        for var in fact.live {
            res.insert(var);
        }
    }

    LivenessFact { live: res }
}

/// ```plain
///       +-1-----+
///       | a = 0 |
///       | b = 1 |
///       +-------+
///        |     |
///        v     v
/// +-2-----+   +-3-----+
/// | c = b |   | c = a |
/// +-------+   +-------+
///        |     |
///        v     v
///       +-4-----+
///       | d = a |
///       +-------+
///
/// in(1)  = {}
/// out(1) = {a, b}
/// in(2)  = {a, b}
/// out(2) = {a}
/// in(3)  = {a}
/// out(3) = {a}
/// in(4)  = {a}
/// out(4) = {}
/// ```
#[test]
fn one_branch() {
    // Build graph
    let b1 = block! {
        1;
        from => ;
        to => 2, 3;
        (0 = 0);
        (1 = 1)
    };

    let b2 = block! {
        2;
        from => 1;
        to => 4;
        (3 = var 1)
    };

    let b3 = block! {
        3;
        from => 1;
        to => 4;
        (3 = var 0)
    };

    let b4 = block! {
        4;
        from => 2, 3;
        to => ;
        (4 = var 0)
    };

    let mut graph = NodeGraph::new(b1);
    graph.insert(b2);
    graph.insert(b3);
    graph.insert_exit(b4);

    // Analyze
    let top = LivenessFact {
        live: FnvHashSet::default(),
    };
    let exit = top.clone();

    let mut analyzer = Analyzer::new_backward(exit, top, trans, join);
    let res = analyzer.solve(&graph);

    // Compare
    let expected = dict![
        BlockId(1) => NodeInfo {
            before: LivenessFact::new(set![]),
            after: LivenessFact::new(set![Variable(0), Variable(1)]),
        },
        BlockId(2) => NodeInfo {
            before: LivenessFact::new(set![Variable(0), Variable(1)]),
            after: LivenessFact::new(set![Variable(0)]),
        },
        BlockId(3) => NodeInfo {
            before: LivenessFact::new(set![Variable(0)]),
            after: LivenessFact::new(set![Variable(0)]),
        },
        BlockId(4) => NodeInfo {
            before: LivenessFact::new(set![Variable(0)]),
            after: LivenessFact::new(set![]),
        }
    ];

    assert_eq!(expected, res);
}

/// ```plain
/// +-1-----+
/// | a = 0 |
/// +-------+
///     |
///     v      
/// +-2-----+  
/// | b = 1 |<-+
/// | c = a |  |
/// +-------+  |
///   |   |    |
///   |   +----+
///   v
/// +-3-----+
/// | d = a |
/// | e = b |
/// +-------+
///
/// in(1)  = {}
/// out(1) = {a}
/// in(2)  = {a}
/// out(2) = {a, b}
/// in(3)  = {a, b}
/// out(3) = {}
/// ```
#[test]
fn one_loop() {
    // Build graph
    let b1 = block! {
        1;
        from => ;
        to => 2;
        (0 = 0)
    };

    let b2 = block! {
        2;
        from => 1, 2;
        to => 2, 3;
        (1 = 1);
        (2 = var 0)
    };

    let b3 = block! {
        3;
        from => 2;
        to => ;
        (3 = var 0);
        (4 = var 1)
    };

    let mut graph = NodeGraph::new(b1);
    graph.insert(b2);
    graph.insert_exit(b3);

    // Analyze
    let top = LivenessFact {
        live: FnvHashSet::default(),
    };
    let exit = top.clone();

    let mut analyzer = Analyzer::new_backward(exit, top, trans, join);
    let res = analyzer.solve(&graph);

    // Compare
    let expected = dict![
        BlockId(1) => NodeInfo {
            before: LivenessFact::new(set![]),
            after: LivenessFact::new(set![Variable(0)]),
        },
        BlockId(2) => NodeInfo {
            before: LivenessFact::new(set![Variable(0)]),
            after: LivenessFact::new(set![Variable(0), Variable(1)]),
        },
        BlockId(3) => NodeInfo {
            before: LivenessFact::new(set![Variable(0), Variable(1)]),
            after: LivenessFact::new(set![]),
        }
    ];

    assert_eq!(expected, res);
}
