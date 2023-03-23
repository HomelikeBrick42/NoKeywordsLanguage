use crate::{
    binding::Type,
    eval::Value,
    nodes::{NodeID, Nodes},
    tokens::{GetLocation, SourceLocation},
};

#[derive(Debug, Clone)]
pub enum BoundNode<'filepath> {
    Block {
        location: SourceLocation<'filepath>,
        end_location: SourceLocation<'filepath>,
        expressions: Vec<NodeID<BoundNode<'filepath>>>,
        result_type: NodeID<Type>,
    },
    Constant {
        location: SourceLocation<'filepath>,
        end_location: SourceLocation<'filepath>,
        typ: NodeID<Type>,
        value: Value<'filepath>,
    },
    Declaration {
        location: SourceLocation<'filepath>,
        end_location: SourceLocation<'filepath>,
        typ: NodeID<Type>,
        value: Option<NodeID<BoundNode<'filepath>>>,
    },
    Type {
        location: SourceLocation<'filepath>,
        end_location: SourceLocation<'filepath>,
        typ: NodeID<Type>,
        type_type: NodeID<Type>,
    },
    Name {
        location: SourceLocation<'filepath>,
        end_location: SourceLocation<'filepath>,
        referenced_node: NodeID<BoundNode<'filepath>>,
    },
    MemberAccess {
        location: SourceLocation<'filepath>,
        end_location: SourceLocation<'filepath>,
        operand: NodeID<BoundNode<'filepath>>,
        member_index: usize,
        result_type: NodeID<Type>,
    },
    Call {
        location: SourceLocation<'filepath>,
        end_location: SourceLocation<'filepath>,
        operand: NodeID<BoundNode<'filepath>>,
        arguments: Vec<NodeID<BoundNode<'filepath>>>,
        result_type: NodeID<Type>,
    },
    Cast {
        location: SourceLocation<'filepath>,
        end_location: SourceLocation<'filepath>,
        to_type: NodeID<Type>,
        from_expressions: Vec<NodeID<BoundNode<'filepath>>>,
    },
    Procedure {
        location: SourceLocation<'filepath>,
        end_location: SourceLocation<'filepath>,
        parameters: Vec<NodeID<BoundNode<'filepath>>>,
        return_type: NodeID<Type>,
        typ: NodeID<Type>,
        body: NodeID<BoundNode<'filepath>>,
    },
}

impl<'filepath> BoundNode<'filepath> {
    pub fn is_constant(&self, nodes: &Nodes<BoundNode<'filepath>>) -> bool {
        match *self {
            BoundNode::Block {
                ref expressions, ..
            } => expressions.iter().all(|&id| nodes[id].is_constant(nodes)),
            BoundNode::Constant { .. } => true,
            BoundNode::Declaration { .. } => false,
            BoundNode::Type { .. } => true,
            BoundNode::Name {
                referenced_node, ..
            } => nodes[referenced_node].is_constant(nodes),
            BoundNode::MemberAccess { operand, .. } => nodes[operand].is_constant(nodes),
            BoundNode::Call {
                operand,
                ref arguments,
                ..
            } => {
                nodes[operand].is_constant(nodes)
                    && arguments.iter().all(|&id| nodes[id].is_constant(nodes))
            }
            BoundNode::Cast {
                ref from_expressions,
                ..
            } => from_expressions
                .iter()
                .all(|&expression| nodes[expression].is_constant(nodes)),
            BoundNode::Procedure { .. } => true,
        }
    }

    pub fn get_type(&self, nodes: &Nodes<BoundNode<'filepath>>) -> NodeID<Type> {
        match *self {
            BoundNode::Block { result_type, .. } => result_type,
            BoundNode::Constant { typ, .. } => typ,
            BoundNode::Declaration { typ, .. } => typ,
            BoundNode::Type { type_type, .. } => type_type,
            BoundNode::Name {
                referenced_node, ..
            } => nodes[referenced_node].get_type(nodes),
            BoundNode::MemberAccess { result_type, .. } => result_type,
            BoundNode::Call { result_type, .. } => result_type,
            BoundNode::Cast { to_type, .. } => to_type,
            BoundNode::Procedure { typ, .. } => typ,
        }
    }
}

impl<'filepath> GetLocation<'filepath> for BoundNode<'filepath> {
    fn get_location(&self) -> SourceLocation<'filepath> {
        match *self {
            BoundNode::Block { location, .. }
            | BoundNode::Constant { location, .. }
            | BoundNode::Declaration { location, .. }
            | BoundNode::Type { location, .. }
            | BoundNode::Name { location, .. }
            | BoundNode::MemberAccess { location, .. }
            | BoundNode::Call { location, .. }
            | BoundNode::Cast { location, .. }
            | BoundNode::Procedure { location, .. } => location,
        }
    }

    fn get_end_location(&self) -> SourceLocation<'filepath> {
        match *self {
            BoundNode::Block { end_location, .. }
            | BoundNode::Constant { end_location, .. }
            | BoundNode::Declaration { end_location, .. }
            | BoundNode::Type { end_location, .. }
            | BoundNode::Name { end_location, .. }
            | BoundNode::MemberAccess { end_location, .. }
            | BoundNode::Call { end_location, .. }
            | BoundNode::Cast { end_location, .. }
            | BoundNode::Procedure { end_location, .. } => end_location,
        }
    }
}
