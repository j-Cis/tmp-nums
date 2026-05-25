use super::cas::Node;

pub fn simplify_equation(node: &Node) -> Node {
    node.simplify()
}
