# creek

Creek is a small, simple library for doing flow analysis.

## Usage

See `tests` for examples of problem definitions. `tests/cfg` contains a simple
node and graph definition.

Creek operates on directed graphs, which must implement the `Graph` trait. The
graph consists of nodes, which must implement the `Node` trait. Note that a
graph must have exactly one entry node and one exit node. These can be the same.
Nodes must be uniquely identified by an id which is `Copy`, `Eq`, and `Hash`.

```rust
struct MyNode {
    id: usize,
    // ...
}

impl creek::Node for MyNode {
    type NodeId = usize;
}

struct MyGraph {
    nodes: std::collections::HashMap<usize, MyNode>,
}

impl creek::Graph<MyNode> for MyGraph {
    fn get(&self, id: usize) -> &MyNode {
        // `get` is only called with `id`s from this graph, so `unwrapping` is
        // ok
        self.nodes.get(id).unwrap()
    }

    // ...
}
```

A problem is defined in terms of two functions, `trans` and `join`, and the type
of fact it operates on. The fact type must implement the `Fact` trait, which
just requires implementing `Clone` and `PartialEq` (waiting on trait aliases to
stabilize).

```rust
#[derive(Clone, PartialEq)]
struct LivenessFact {
    live_vars: std::collections::HashSet<usize>,
}

impl Fact for LivenessFact {}
```

`trans` is a function which takes a node and a fact for that node, and produces
a new fact for that node. In a forwards problem, `trans` gives you the `after`
fact in terms of a node and its `before` fact.

`join` is a function which takes multiple facts and combines them into one. In
a forwards problem, this combines the `after` facts for a node's predecessors to
create that node's `before` fact.

```rust
fn trans(node: &MyNode, fact: LivenessFact) -> LivenessFact {
    // ...
}

fn join(facts: Vec<LivenessFact>) -> LivenessFact {
    // ...
}
```

Finally, to create an `Analyzer` we need a top fact, which is a fact with the
property that for any fact `f` , `join(top, f) == f`. If the fact is a set and
`join` is the union operation, then the top fact would be the empty set.

```rust
let top = LivenessFact {
    lives: std::collections::HashSet::new()
};

let mut analyzer = Analyzer::new_backwards(top, trans, join);
```

The analyzer gives back `NodeInfo`s for all nodes it can reach after a solve.

```rust
let res = analyzer.solve(some_graph);
for (_, info) in res.iter() {
    // use the info.before and info.after facts here
}
```

## Dependencies

By default, Creek uses [fnv](https://doc.servo.org/fnv/) instead of the standard
[`SipHasher`](https://doc.rust-lang.org/std/hash/struct.SipHasher.html) as it is
more performant on small keys, such as integers. This can be disabled with the
`no-deps` feature.
