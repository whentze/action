use fnv::FnvHashMap;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use definitions::*;
use module::Module;

#[derive(Clone)]
pub struct Input(Rc<Cell<Chunk>>);
impl Input {
    pub fn get(&self) -> Chunk {
        self.0.as_ref().get()
    }
}
impl<'a> From<&'a Output> for Input {
    fn from(o: &'a Output) -> Input {
        Input(o.0.clone())
    }
}

#[derive(Clone)]
pub struct Output(Rc<Cell<Chunk>>);
impl Output {
    pub fn put(&self, val: Chunk) {
        self.0.set(val)
    }
}

static NEXT_ID : AtomicUsize = AtomicUsize::new(0);
fn new_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

struct Node {
    module:  Box<Module>,
    inputs:  Vec<Input>,
    outputs: Vec<Output>,
}

pub struct Graph {
    nodes: FnvHashMap<usize, Node>,
    buf: Vec<Chunk>,
    null_input: Input,
    null_output: Output,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: FnvHashMap::default(),
            buf: vec![Chunk::default(); 16],
            null_input: Input(Rc::new(Cell::new(Chunk::default()))),
            null_output: Output(Rc::new(Cell::new(Chunk::default()))),
        }
    }
    pub fn add_module<M: Module + 'static>(&mut self, module: M) -> usize {
        let module = Box::new(module);
        let inputs = vec![self.null_input.clone(); module.num_inputs()];
        let outputs = vec![self.null_output.clone(); module.num_outputs()];
        let node = Node { module, inputs, outputs };
        let id = new_id();
        self.nodes.insert(id, node);
        id
    }
    pub fn connect(&mut self, src: PortSpec, dst: PortSpec) -> Result<()> {
        let src_module = self.nodes.remove(&src.0).ok_or(err_msg("Source module not found"))?;
        let dst_module = self.nodes.get_mut(&dst.0).ok_or(err_msg("Destination module not found"))?;
        dst_module.inputs[dst.1] = Input::from(&src_module.outputs[src.1]);
        self.nodes.insert(src.0, src_module);
        Ok(())
    }
    pub fn run(&mut self) {
        for n in self.nodes.values_mut() {
            n.module.run(&n.inputs, &n.outputs);
        }
    }
}