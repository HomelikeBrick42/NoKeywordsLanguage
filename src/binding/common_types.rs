use crate::{
    binding::Type,
    nodes::{NodeID, Nodes},
};
use std::collections::HashMap;

#[allow(clippy::type_complexity)]
pub struct CommonTypes {
    pub typ: NodeID<Type>,
    pub void: NodeID<Type>,
    pub int: NodeID<Type>,
    pub uint: NodeID<Type>,
    pub u8: NodeID<Type>,
    /// HashMap from `inner_type` to `Slice { inner_type }`
    pub slice_types: HashMap<NodeID<Type>, NodeID<Type>>,
    /// HashMap from `pointed_to` to `Pointer { pointed_to }`
    pub pointer_types: HashMap<NodeID<Type>, NodeID<Type>>,
    /// HashMap from `pointed_to` to `MultiPointer { pointed_to }`
    pub multipointer_types: HashMap<NodeID<Type>, NodeID<Type>>,
    /// HashMap from `parameters` to HashMap from `return_type` to `Procedure { parameters, return_type }`
    pub procedure_types: HashMap<Vec<NodeID<Type>>, HashMap<NodeID<Type>, NodeID<Type>>>,
}

impl CommonTypes {
    pub fn get_slice(&mut self, types: &mut Nodes<Type>, inner_type: NodeID<Type>) -> NodeID<Type> {
        if let Some(&slice) = self.slice_types.get(&inner_type) {
            slice
        } else {
            let slice = types.insert(Type::Slice { inner_type });
            let previous = self.slice_types.insert(inner_type, slice);
            assert!(previous.is_none());
            slice
        }
    }

    pub fn get_pointer(
        &mut self,
        types: &mut Nodes<Type>,
        pointed_to: NodeID<Type>,
    ) -> NodeID<Type> {
        if let Some(&pointer) = self.pointer_types.get(&pointed_to) {
            pointer
        } else {
            let pointer = types.insert(Type::Pointer { pointed_to });
            let previous = self.pointer_types.insert(pointed_to, pointer);
            assert!(previous.is_none());
            pointer
        }
    }

    pub fn get_multipointer(
        &mut self,
        types: &mut Nodes<Type>,
        pointed_to: NodeID<Type>,
    ) -> NodeID<Type> {
        if let Some(&multipointer) = self.multipointer_types.get(&pointed_to) {
            multipointer
        } else {
            let multipointer = types.insert(Type::Multipointer { pointed_to });
            let previous = self.multipointer_types.insert(pointed_to, multipointer);
            assert!(previous.is_none());
            multipointer
        }
    }

    pub fn get_procedure(
        &mut self,
        types: &mut Nodes<Type>,
        parameters: &[NodeID<Type>],
        return_type: NodeID<Type>,
    ) -> NodeID<Type> {
        let inner = if let Some(inner) = self.procedure_types.get_mut(parameters) {
            inner
        } else {
            self.procedure_types.entry(parameters.to_vec()).or_default()
        };

        if let Some(&procedure) = inner.get(&return_type) {
            procedure
        } else {
            let procedure = types.insert(Type::Procedure {
                parameters: parameters.to_vec(),
                return_type,
            });
            inner.insert(return_type, procedure);
            procedure
        }
    }
}
