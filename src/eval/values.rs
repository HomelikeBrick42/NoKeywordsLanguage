use crate::{
    binding::{BoundNode, Type},
    nodes::NodeID,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value<'filepath> {
    Type {
        typ: NodeID<Type>,
    },
    Procedure {
        procedure: NodeID<BoundNode<'filepath>>,
    },
}
