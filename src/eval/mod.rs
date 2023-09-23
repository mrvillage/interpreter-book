use std::any::TypeId;

use crate::{object::Object, ExpressionStatement, IntegerLiteral, Node, NodeType, Program};

mod tests;

// Rust made this impossible to finish without rewriting the entire parser so here we are
pub fn eval(node: Box<dyn Node>) -> Object {
    match node.node_type() {
        // NodeType::ExpressionStatement => eval(
        //     node.as_any()
        //         .downcast_ref::<ExpressionStatement>()
        //         .unwrap()
        //         .expression,
        // ),
        _ => Object::Null,
    }
}
