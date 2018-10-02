use definitions::*;
use module::Module;
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::Direction;

#[derive(Debug)]
struct Node {
    module: Box<Module>,
    input: Vec<Chunk>,
    output: Vec<Chunk>,
}

pub type PortNum = usize;
pub type PortAddr = (NodeIndex, PortNum);

#[derive(Default, Debug)]
pub struct Graph(DiGraph<Node, (PortNum, PortNum)>);

impl Graph {
    pub fn new() -> Self {
        Graph(DiGraph::with_capacity(128, 512))
    }
    pub fn insert_module<M: Module + 'static>(&mut self, module: M) -> NodeIndex {
        let module = Box::new(module);
        let input = vec![Chunk::default(); module.num_inputs()];
        let output = vec![Chunk::default(); module.num_outputs()];
        let node = Node {
            module,
            input,
            output,
        };
        self.0.add_node(node)
    }
    pub fn remove_module(&mut self, id: NodeIndex) -> Result<Box<Module>> {
        self.0
            .remove_node(id)
            .ok_or_else(|| err_msg("No Module with that Id exists."))
            .map(|node| node.module)
    }
    pub fn connect(&mut self, src: PortAddr, dst: PortAddr) -> Result<EdgeIndex> {
        let src_node = self
            .0
            .node_weight(src.0)
            .ok_or_else(|| err_msg(format!("No module with id {:?} exists.", src.0)))?;
        if src_node.module.num_outputs() <= src.1 {
            return Err(err_msg("Port number for input module is too high."));
        }
        let dst_node = self
            .0
            .node_weight(dst.0)
            .ok_or_else(|| err_msg(format!("No module with id {:?} exists.", dst.0)))?;
        if dst_node.module.num_inputs() <= dst.1 {
            return Err(err_msg("Port number for output module is too high."));
        }
        Ok(self.0.add_edge(src.0, dst.0, (src.1, dst.1)))
    }
    pub fn disconnect(&mut self, edge: EdgeIndex) -> Result<(usize, usize)> {
        self.0
            .remove_edge(edge)
            .ok_or_else(|| err_msg("Edge to be removed does not exist."))
    }
    pub fn run(&mut self) {
        for ni in self.0.node_indices() {
            let node = self.0.node_weight_mut(ni).unwrap();
            node.module.process_chunks(&node.input, &mut node.output);
            let mut neighbors = self.0.neighbors_directed(ni, Direction::Outgoing).detach();
            while let Some((edge, neighbor)) = neighbors.next(&self.0) {
                let &(src_port, dst_port) = self.0.edge_weight(edge).unwrap();
                let (src_node, dst_node) = self.0.index_twice_mut(ni, neighbor);
                dst_node.input[dst_port] = src_node.output[src_port];
            }
        }
    }
}
