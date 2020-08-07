//! Liveness analysis finds all variables which may be used after a given point.
//!
//! Liveness analysis is a forwards problem with these functions
//!
//! ```plain
//! trans(b) = union(gen(b), in(b) - kill(b))
//! join = union
//! ```
//!
//! where `gen(b)` gives the variables used, and `kill(b)` gives the variables
//! reassigned.

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
            Statement::Declare(_) => {}
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

    let mut analyzer = Analyzer::new_backward(top, trans, join);
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

    let mut analyzer = Analyzer::new_backward(top, trans, join);
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

///       +-1-----+
///       | k = 2 |
///       +-------+
///        |     |
///        v     v
/// +-2-----+   +-3-----+
/// | a = k |   | a = k |
/// +-------+   +-------+
///     |           |
///     v           v
/// +-4-----+   +-5-----+
/// | x = 5 |   | x = 8 |
/// +-------+   +-------+
///        |     |
///        v     v
///       +-6-----+
///       | k = a |<-+
///       +-------+  |
///         |   |    |
///    +----+   |    |
///    |        v    |
///    |  +-7-----+  |
///    |  | b = 2 |  |
///    |  +-------+  |
///    |      |      |
///    |      v      |
///    |  +-8-----+  |
///    |  | x = a |  |
///    |  | y = b |  |
///    |  +-------+  |
///    |      |      |
///    |      v      |
///    |  +-9-----+  |
///    |  | k = k |  |
///    |  +-------+  |
///    |        |    |
///    +----+   +----+
///         |
///         v
///       +-10----+
///       | m = a |
///       | n = x |
///       +-------+
///
/// in(1)   = {}
/// out(1)  = {k}
/// in(2)   = {k}
/// out(2)  = {a}
/// in(3)   = {k}
/// out(3)  = {a}
/// in(4)   = {a}
/// out(4)  = {a, x}
/// in(5)   = {a}
/// out(5)  = {a, x}
/// in(6)   = {a, x}
/// out(6)  = {a, k, x}
/// in(7)   = {a, k}
/// out(7)  = {a, b, k}
/// in(8)   = {a, b, k}
/// out(8)  = {a, k, x}
/// in(9)   = {a, k, x}
/// out(9)  = {a, x}
/// in(10)  = {a, x}
/// out(10) = {}
#[test]
fn branch_and_loop() {
    // Build blocks
    let mut graph = NodeGraph::new(block! {
        1;
        from => ;
        to => 2, 3;
        (0 = 2)
    });

    graph.insert(block! {
        2;
        from => 1;
        to => 4;
        (1 = var 0)
    });

    graph.insert(block! {
        3;
        from => 1;
        to => 5;
        (1 = var 0)
    });

    graph.insert(block! {
        4;
        from => 2;
        to => 6;
        (2 = 5)
    });

    graph.insert(block! {
        5;
        from => 3;
        to => 6;
        (2 = 8)
    });

    graph.insert(block! {
        6;
        from => 4, 5, 9;
        to => 7, 10;
        (0 = var 1)
    });

    graph.insert(block! {
        7;
        from => 6;
        to => 8;
        (3 = 2)
    });

    graph.insert(block! {
        8;
        from => 7;
        to => 9;
        (2 = var 1);
        (4 = var 3)
    });

    graph.insert(block! {
        9;
        from => 8;
        to => 6;
        (0 = var 0)
    });

    graph.insert_exit(block! {
        10;
        from => 6;
        to => ;
        (5 = var 1);
        (6 = var 2)
    });

    // Analyze
    let top = LivenessFact {
        live: FnvHashSet::default(),
    };

    let mut analyzer = Analyzer::new_backward(top, trans, join);
    let res = analyzer.solve(&graph);

    // Compare
    let expected = dict![
        BlockId(1) => NodeInfo {
            before: LivenessFact::new(set![]),
            after: LivenessFact::new(set![Variable(0)]),
        },
        BlockId(2) => NodeInfo {
            before: LivenessFact::new(set![Variable(0)]),
            after: LivenessFact::new(set![Variable(1)]),
        },
        BlockId(3) => NodeInfo {
            before: LivenessFact::new(set![Variable(0)]),
            after: LivenessFact::new(set![Variable(1)]),
        },
        BlockId(4) => NodeInfo {
            before: LivenessFact::new(set![Variable(1)]),
            after: LivenessFact::new(set![Variable(1), Variable(2)]),
        },
        BlockId(5) => NodeInfo {
            before: LivenessFact::new(set![Variable(1)]),
            after: LivenessFact::new(set![Variable(1), Variable(2)]),
        },
        BlockId(6) => NodeInfo {
            before: LivenessFact::new(set![Variable(1), Variable(2)]),
            after: LivenessFact::new(set![Variable(0), Variable(1), Variable(2)]),
        },
        BlockId(7) => NodeInfo {
            before: LivenessFact::new(set![Variable(0), Variable(1)]),
            after: LivenessFact::new(set![Variable(0), Variable(1), Variable(3)]),
        },
        BlockId(8) => NodeInfo {
            before: LivenessFact::new(set![Variable(0), Variable(1), Variable(3)]),
            after: LivenessFact::new(set![Variable(0), Variable(1), Variable(2)]),
        },
        BlockId(9) => NodeInfo {
            before: LivenessFact::new(set![Variable(0), Variable(1), Variable(2)]),
            after: LivenessFact::new(set![Variable(1), Variable(2)]),
        },
        BlockId(10) => NodeInfo {
            before: LivenessFact::new(set![Variable(1), Variable(2)]),
            after: LivenessFact::new(set![]),
        }
    ];

    assert_eq!(expected, res);
}
