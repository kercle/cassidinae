// use crate::parser::ast::{AstNode, BinaryOp, UnaryOp};

// pub fn coalesce_constants(node: &mut AstNode) {
//     match node {
//         AstNode::Block(nodes) => {
//             for child in nodes.iter_mut() {
//                 coalesce_constants(child);
//             }
//         }

//     }
// }

// pub fn subtractions_to_additions(node: &mut AstNode) {
//     match node {
//         AstNode::Block(nodes) => {
//             for child in nodes.iter_mut() {
//                 subtractions_to_additions(child);
//             }
//         }
//         AstNode::BinaryNode { op, lhs, rhs } => {
//             if *op == BinaryOp::Sub {
//                 *op = BinaryOp::Add;
//                 let new_rhs = AstNode::UnaryNode { 
//                     op: UnaryOp::Negate,    
//                 }
//         }
//         _ => {}
//     }
// }