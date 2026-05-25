use crate::api::Node; // Pobieramy Node przez nasze API (lub bezpośrednio z cas)

pub fn simplify_equation(node: &Node) -> Node {
    node.simplify()
}
