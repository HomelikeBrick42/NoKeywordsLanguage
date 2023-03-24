use crate::nodes::{NodeID, Nodes};

#[derive(Debug, Clone)]
pub enum Type {
    Type,
    Void,
    Int,
    UInt,
    U8,
    Slice {
        inner_type: NodeID<Type>,
    },
    Pointer {
        pointed_to: NodeID<Type>,
    },
    Multipointer {
        pointed_to: NodeID<Type>,
    },
    Procedure {
        parameters: Vec<NodeID<Type>>,
        return_type: NodeID<Type>,
    },
}

impl Type {
    pub fn pretty_print(&self, types: &Nodes<Type>) -> String {
        match *self {
            Type::Type => "type".to_string(),
            Type::Void => "void".to_string(),
            Type::Int => "int".to_string(),
            Type::UInt => "uint".to_string(),
            Type::U8 => "u8".to_string(),
            Type::Slice { inner_type } => format!("[]{}", types[inner_type].pretty_print(types)),
            Type::Pointer { pointed_to } => format!("^{}", types[pointed_to].pretty_print(types)),
            Type::Multipointer { pointed_to } => {
                format!("[^]{}", types[pointed_to].pretty_print(types))
            }
            Type::Procedure {
                ref parameters,
                return_type,
            } => {
                let mut result = "(".to_string();
                for (i, &parameter) in parameters.iter().enumerate() {
                    if i > 0 {
                        result += ", ";
                    }
                    result += &types[parameter].pretty_print(types);
                }
                result += ") -> ";
                result += &types[return_type].pretty_print(types);
                result
            }
        }
    }

    pub fn get_size(&self, types: &Nodes<Type>) -> usize {
        _ = types;
        match *self {
            Type::Type => 8,
            Type::Void => 0,
            Type::Int => 8,
            Type::UInt => 8,
            Type::U8 => 1,
            Type::Slice { .. } => 16,
            Type::Pointer { .. } => 8,
            Type::Multipointer { .. } => 8,
            Type::Procedure { .. } => 8,
        }
    }
}
