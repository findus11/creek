//! Definite assignment analysis finds all variables which are definitely
//! assigned at a program point.
//!
//! This implements a slight variation, where we find the variables which are
//! possibly unassigned. It is a forwards problem with the following functions
//!
//! ```plain
//! trans(b) = union(gen(b), in(b) - kill(b))
//! join = union
//! ```
//!
//! where `gen(b)` gives the variables which are declared, and `kill(b)` gives
//! the variables which are defined. A variable shouldn't be declared and
//! defined in the same block in this system.

mod cfg;

use cfg::*;
use creek::{Analyzer, Fact, NodeInfo};
use fnv::FnvHashSet;

#[derive(Clone, Debug, PartialEq)]
struct AssignmentFact {
    uninit: FnvHashSet<Variable>,
}

impl AssignmentFact {
    pub fn new(uninit: FnvHashSet<Variable>) -> Self {
        Self { uninit }
    }
}

impl Fact for AssignmentFact {}

/// ```plain
/// trans(b) = union(gen(b), in(b) - kill(b))
/// ```
fn trans(block: &Block, mut fact: AssignmentFact) -> AssignmentFact {
    for stmt in block.stmts.iter() {
        match stmt {
            Statement::Declare(var) => {
                fact.uninit.insert(var.clone());
            }
            Statement::ConstAssign(var, _) => {
                fact.uninit.remove(&var);
            }
            Statement::VarAssign(var, _) => {
                fact.uninit.remove(&var);
            }
        }
    }

    fact
}

/// ```plain
/// join = union
/// ```
fn join(facts: Vec<AssignmentFact>) -> AssignmentFact {
    let mut res = FnvHashSet::default();

    for fact in facts {
        for var in fact.uninit {
            res.insert(var);
        }
    }

    AssignmentFact { uninit: res }
}

/// ```plain
/// +-1-----+
/// | var a |
/// +-------+
///   |   |
///   |   v
///   | +-2-----+
///   | | a = 1 |
///   | +-------+
///   |   |
///   v   v
/// +-3-----+
/// | b = a |
/// +-------+
///
/// in(1)  = {}
/// out(1) = {a}
/// in(2)  = {a}
/// out(2) = {}
/// in(3)  = {a}
/// out(3) = {a}
/// ```
#[test]
fn one_branch() {
    // Build blocks
    let b1 = block! {
        1;
        from => ;
        to => 2, 3;
        (var 0)
    };

    let b2 = block! {
        2;
        from => 1;
        to => 3;
        (0 = 1)
    };

    let b3 = block! {
        3;
        from => 1, 2;
        to => ;
        (1 = var 0)
    };

    let mut graph = NodeGraph::new(b1);
    graph.insert(b2);
    graph.insert_exit(b3);

    // Analyze
    let enter = AssignmentFact::new(set![]);
    let top = enter.clone();

    let mut analyzer = Analyzer::new_forward(enter, top, trans, join);
    let res = analyzer.solve(&graph);

    // Compare
    let expected = dict![
        BlockId(1) => NodeInfo {
            before: AssignmentFact::new(set![]),
            after: AssignmentFact::new(set![Variable(0)]),
        },
        BlockId(2) => NodeInfo {
            before: AssignmentFact::new(set![Variable(0)]),
            after: AssignmentFact::new(set![]),
        },
        BlockId(3) => NodeInfo {
            before: AssignmentFact::new(set![Variable(0)]),
            after: AssignmentFact::new(set![Variable(0)]),
        }
    ];

    assert_eq!(expected, res);
}
