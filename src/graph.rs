use rayon::prelude::*;
use rayon_hash::{HashMap, HashSet};
use fnv::FnvBuildHasher;
use std::sync::atomic::{AtomicUsize, Ordering};

use definitions::*;
use module::Module;

static NEXT_ID : AtomicUsize = AtomicUsize::new(0);
fn new_id() -> ModuleId {
    ModuleId(NEXT_ID.fetch_add(1, Ordering::SeqCst))
}

struct Node {
    module: Box<Module>,
    input:  Vec<Chunk>,
    output: Vec<Chunk>,
}

#[derive(PartialEq, Eq, Hash)]
struct Connection {
    src: PortAddr,
    dst: PortAddr,
}

pub struct Graph {
    nodes: HashMap<ModuleId, Node, FnvBuildHasher>,
    connections : HashSet<Connection, FnvBuildHasher>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::with_hasher(Default::default()),
            connections: HashSet::with_hasher(Default::default()),
        }
    }
    pub fn insert_module<M: Module + 'static>(&mut self, module: M) -> ModuleId {
        let id = new_id();
        let module = Box::new(module);
        let input  = vec![Chunk::default(); module.num_inputs()];
        let output = vec![Chunk::default(); module.num_outputs()];
        let node = Node {
            module, input, output
        };
        assert!(self.nodes.insert(id, node).is_none());
        id
    }
    pub fn remove_module(&mut self, id: ModuleId) -> Result<()> {
        if self.nodes.remove(&id).is_some() {
            self.connections.retain(|&Connection{src, dst}| src.0 != id && dst.0 != id);
            Ok(())
        } else {
            Err(err_msg("No Module with that Id exists."))
        }
    }
    pub fn connect(&mut self, src: PortAddr, dst: PortAddr) -> Result<()> {
        if self.connections.insert(Connection {src, dst}) {
            Ok(())
        } else {
            Err(err_msg("Connection already exists."))
        }
    }
    pub fn disconnect(&mut self, src: PortAddr, dst: PortAddr) -> Result<()> {
        if self.connections.remove(&Connection {src, dst}) {
            Ok(())
        } else {
            Err(err_msg("Connection does not exist."))
        }
    }
    pub fn run(&mut self) {
        self.nodes.par_values_mut().for_each(|n| {
            n.module.process_chunks(&n.input, &mut n.output);
        });
        for &Connection{src, dst} in &self.connections {
            let chunk = self.nodes.get(&src.0).unwrap().output[src.1];
            let dst_node = self.nodes.get_mut(&dst.0).unwrap();
            dst_node.input[dst.1] = chunk;
        }
    }
}