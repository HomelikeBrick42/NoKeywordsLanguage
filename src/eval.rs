mod values;

pub use values::*;

use crate::{
    binding::BoundNode,
    nodes::{NodeID, Nodes},
};

pub fn eval_bound_node<'filepath>(
    node: NodeID<BoundNode<'filepath>>,
    nodes: &mut Nodes<BoundNode<'filepath>>,
) -> Value<'filepath> {
    match nodes[node] {
        BoundNode::Block {
            ref expressions, ..
        } => {
            _ = expressions;
            todo!()
        }
        BoundNode::Constant { ref value, .. } => value.clone(),
        BoundNode::Declaration { typ, value, .. } => {
            _ = typ;
            _ = value;
            todo!()
        }
        BoundNode::Type { typ, .. } => Value::Type { typ },
        BoundNode::Name {
            referenced_node, ..
        } => eval_bound_node(referenced_node, nodes),
        BoundNode::MemberAccess {
            operand,
            member_index,
            ..
        } => {
            _ = operand;
            _ = member_index;
            todo!()
        }
        BoundNode::Call {
            operand,
            ref arguments,
            ..
        } => {
            _ = operand;
            _ = arguments;
            todo!();
        }
        BoundNode::Cast {
            to_type,
            ref from_expressions,
            ..
        } => {
            _ = to_type;
            _ = from_expressions;
            todo!()
        }
        BoundNode::Procedure { .. } => Value::Procedure { procedure: node },
    }
}
