use k_engine_core::{NodeData, Node, Visitor};
use k_engine_macros::NodeData;

#[derive(NodeData)]
struct MultiplyNode {
    pub factor: i32,
}

#[derive(NodeData)]
struct AddNode {
    pub value: i32,
}

struct CalcVisitor {
    pub res: i32,
}

impl Visitor<MultiplyNode> for CalcVisitor {
    fn visit(&mut self, data: &MultiplyNode) {
        self.res *= data.factor;
    }
}

impl Visitor<AddNode> for CalcVisitor {
    fn visit(&mut self, data: &AddNode) {
        self.res += data.value;
    }
}

fn main() {
    let mut root = Node::empty();

    let childa_data = MultiplyNode { factor: 2 };
    let childa = Node::new(childa_data);

    let mut childb = Node::empty();
    childb.enabled = true;

    let childc_data = AddNode { value: 1 };
    let childc = Node::new(childc_data);
    childb.add_child(childc);

    root.add_child(childa);
    root.add_child(childb);

    let mut sum_visitor = CalcVisitor { res: 1 };
    root.accept(&mut sum_visitor);
    println!("{}", sum_visitor.res);
}

