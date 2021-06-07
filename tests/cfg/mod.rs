//! A simple control flow graph. A `NodeGraph` consists of one or more blocks,
//! each of which consists of multiple statements. Each statement is either a
//! constant assignment, like `x = 5`, or a variable assignment, like `x = a`.

#![allow(dead_code)]

pub mod macros;

use creek::{Graph, Node};
use fnv::FnvHashMap;

/// A variable with a unique id
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Variable(pub usize);

/// A simple statement, which is either a constant assignment (`x = 5`) or a
/// variable assignment (`x = a`)
#[derive(Debug)]
pub enum Statement {
    Declare(Variable),
    ConstAssign(Variable, i32),
    VarAssign(Variable, Variable),
}

/// The unique id for a block
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockId(pub usize);

/// A sequence of statements
#[derive(Debug)]
pub struct Block {
    pub id: BlockId,
    pub stmts: Vec<Statement>,
    pub preds: Vec<BlockId>,
    pub succs: Vec<BlockId>,
}

impl Node for Block {
    type NodeId = BlockId;
}

#[derive(Debug)]
pub struct NodeGraph {
    blocks: FnvHashMap<BlockId, Block>,
    block_ids: Vec<BlockId>,
    entry: BlockId,
    exit: BlockId,
}

impl NodeGraph {
    /// Create a `NodeGraph` with the given initial block
    pub fn new(block: Block) -> Self {
        let mut graph = Self {
            blocks: FnvHashMap::default(),
            block_ids: Vec::new(),
            entry: block.id,
            exit: block.id,
        };

        graph.block_ids.push(block.id);
        graph.blocks.insert(block.id, block);
        graph
    }

    /// Insert a block
    pub fn insert(&mut self, block: Block) {
        self.block_ids.push(block.id);
        match self.blocks.insert(block.id, block) {
            Some(block) => panic!("{:?}", block.id),
            None => {}
        }
    }

    /// Insert an entry block
    pub fn insert_entry(&mut self, block: Block) {
        self.block_ids.push(block.id);
        self.entry = block.id;
        match self.blocks.insert(block.id, block) {
            Some(block) => panic!("{:?}", block.id),
            None => {}
        }
    }

    /// Insert an exit block
    pub fn insert_exit(&mut self, block: Block) {
        self.block_ids.push(block.id);
        self.exit = block.id;
        match self.blocks.insert(block.id, block) {
            Some(block) => panic!("{:?}", block.id),
            None => {}
        }
    }
}

impl Graph<Block> for NodeGraph {
    fn get(&self, id: BlockId) -> &Block {
        self.blocks.get(&id).unwrap()
    }

    fn get_entry(&self) -> BlockId {
        self.entry
    }

    fn get_exit(&self) -> BlockId {
        self.exit
    }

    fn get_preds(&self, id: BlockId) -> &[BlockId] {
        &self.get(id).preds
    }

    fn get_succs(&self, id: BlockId) -> &[BlockId] {
        &self.get(id).succs
    }

    fn get_all_node_ids(&self) -> &[BlockId] {
        &self.block_ids
    }
}
