use downcast_rs::{Downcast, impl_downcast};

pub trait NodeData : Downcast {}
impl_downcast!(NodeData);

pub struct Node {
    data: Option<Box<dyn NodeData>>,
    pub enabled: bool,
    children: Vec<Node>,
}

struct EmptyData;
impl NodeData for EmptyData {}

impl Node {
    pub fn empty() -> Self {
        Self {
            data: None,
            enabled: true,
            children: Vec::new(),
        }
    }

    pub fn new<T: NodeData + 'static>(data: T) -> Self {
        Self { 
            data: Some(Box::new(data)),
            enabled: true,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn data<T: NodeData>(&self) -> Option<&T> {
       self.data.as_ref().and_then(|d| d.downcast_ref())
    }

    pub fn data_mut<T: NodeData>(&mut self) -> Option<&mut T> {
        self.data.as_mut().and_then(|d| d.downcast_mut())
    }

    pub fn accept<T: NodeData, V: Visitor<T>>(&self, visitor: &mut V) {
        if !self.enabled {
            return;
        }
        if let Some(data) = self.data::<T>() {
            visitor.visit(data);
        }
        for child in &self.children {
            child.accept(visitor);
        }
    }
}

pub trait Visitor<T: NodeData> {
    fn visit(&mut self, data: &T);
}