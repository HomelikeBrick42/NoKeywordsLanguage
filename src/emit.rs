use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

use crate::{
    binding::{BoundNode, Type},
    eval::Value,
    nodes::{NodeID, Nodes},
};

const STACK_ALIGNMENT: usize = 16;

pub fn emit_file<'filepath>(
    root: NodeID<BoundNode<'filepath>>,
    main_procedure: NodeID<BoundNode<'filepath>>,
    nodes: &Nodes<BoundNode<'filepath>>,
    types: &Nodes<Type>,
    f: &mut dyn Write,
) -> std::io::Result<()> {
    writeln!(f, "global main")?;
    writeln!(f)?;
    writeln!(f, "section .text")?;
    writeln!(f, "main:")?;
    writeln!(f, "    push rbp")?;
    writeln!(f, "    mov rbp, rsp")?;
    writeln!(f, "    call _{}", main_procedure.into_inner())?;
    writeln!(f, "    mov rax, 0")?;
    writeln!(f, "    mov rsp, rbp")?;
    writeln!(f, "    pop rbp")?;
    writeln!(f, "    ret")?;
    let mut procedures = HashSet::new();
    fn get_all_procedures<'filepath>(
        node: NodeID<BoundNode<'filepath>>,
        nodes: &Nodes<BoundNode<'filepath>>,
        procedures: &mut HashSet<NodeID<BoundNode<'filepath>>>,
        walked_nodes: &mut HashSet<NodeID<BoundNode<'filepath>>>,
    ) {
        if walked_nodes.contains(&node) {
            return;
        }
        walked_nodes.insert(node);
        match nodes[node] {
            BoundNode::Block {
                ref expressions, ..
            } => {
                for &expression in expressions {
                    get_all_procedures(expression, nodes, procedures, walked_nodes);
                }
            }
            BoundNode::Constant { ref value, .. } => match *value {
                Value::Type { .. } => {}
                Value::Procedure { procedure } => {
                    get_all_procedures(procedure, nodes, procedures, walked_nodes);
                }
            },
            BoundNode::Declaration { value, .. } => {
                if let Some(value) = value {
                    get_all_procedures(value, nodes, procedures, walked_nodes);
                }
            }
            BoundNode::Type { .. } => {}
            BoundNode::Name {
                referenced_node, ..
            } => get_all_procedures(referenced_node, nodes, procedures, walked_nodes),
            BoundNode::MemberAccess { operand, .. } => {
                get_all_procedures(operand, nodes, procedures, walked_nodes);
            }
            BoundNode::Call {
                operand,
                ref arguments,
                ..
            } => {
                get_all_procedures(operand, nodes, procedures, walked_nodes);
                for &argument in arguments {
                    get_all_procedures(argument, nodes, procedures, walked_nodes);
                }
            }
            BoundNode::Cast {
                ref from_expressions,
                ..
            } => {
                for &expression in from_expressions {
                    get_all_procedures(expression, nodes, procedures, walked_nodes);
                }
            }
            BoundNode::Procedure {
                ref parameters,
                body,
                ..
            } => {
                procedures.insert(node);
                for &parameter in parameters {
                    get_all_procedures(parameter, nodes, procedures, walked_nodes);
                }
                get_all_procedures(body, nodes, procedures, walked_nodes);
            }
        }
    }
    get_all_procedures(root, nodes, &mut procedures, &mut HashSet::new());
    get_all_procedures(main_procedure, nodes, &mut procedures, &mut HashSet::new());
    for procedure in procedures {
        writeln!(f)?;
        writeln!(f, "section .text")?;
        writeln!(f, "_{}:", procedure.into_inner())?;
        writeln!(f, "    push rbp")?;
        writeln!(f, "    mov rbp, rsp")?;
        let mut next_local_variable_location = 0;
        let mut local_variable_locations = HashMap::new();
        let BoundNode::Procedure {
            ref parameters,
            return_type,
            body,
            ..
        } = nodes[procedure] else { unreachable!() };

        // TODO: fix this
        assert!(types[return_type].get_size(types) <= 8);

        for &parameter in parameters {
            compute_local_variable_locations(
                parameter,
                nodes,
                types,
                &mut next_local_variable_location,
                &mut local_variable_locations,
            );
        }
        let mut space_to_allocate = next_local_variable_location;
        space_to_allocate += (space_to_allocate + STACK_ALIGNMENT - 1) % STACK_ALIGNMENT;
        writeln!(f, "    sub rsp, {}", space_to_allocate)?;

        assert_eq!(parameters.len(), 1);
        for &parameter in parameters {
            let offset = local_variable_locations[&parameter];
            let size = types[nodes[parameter].get_type(nodes)].get_size(types);
            if size > 8 {
                let mut i = 0;
                while i < size {
                    match size - i {
                        8.. => {
                            writeln!(f, "    mov rax, [rcx + {i}]")?;
                            writeln!(f, "    mov qword [rbp - {}], rax", offset + i)?;
                            i += 8;
                        }
                        4.. => {
                            writeln!(f, "    mov eax, [rcx + {i}]")?;
                            writeln!(f, "    mov dword [rbp - {}], eax", offset + i)?;
                            i += 4;
                        }
                        2.. => {
                            writeln!(f, "    mov ax, [rcx + {i}]")?;
                            writeln!(f, "    mov word [rbp - {}], ax", offset + i)?;
                            i += 2;
                        }
                        _ => {
                            writeln!(f, "    mov al, [rcx + {i}]")?;
                            writeln!(f, "    mov byte [rbp - {}], al", offset + i)?;
                            i += 1;
                        }
                    }
                }
            } else {
                todo!()
            }
        }

        emit(
            body,
            nodes,
            types,
            f,
            &mut next_local_variable_location,
            &mut local_variable_locations,
        )?;
        writeln!(f, "    add rsp, {}", space_to_allocate)?;
        writeln!(f, "    mov rsp, rbp")?;
        writeln!(f, "    pop rbp")?;
        writeln!(f, "    ret")?;
    }
    Ok(())
}

struct EmitInfo {
    l_value: bool,
}

fn emit<'filepath>(
    node: NodeID<BoundNode<'filepath>>,
    nodes: &Nodes<BoundNode<'filepath>>,
    types: &Nodes<Type>,
    f: &mut dyn Write,
    next_local_variable_location: &mut usize,
    local_variable_locations: &mut HashMap<NodeID<BoundNode<'filepath>>, usize>,
) -> std::io::Result<EmitInfo> {
    Ok(match nodes[node] {
        BoundNode::Block {
            ref expressions, ..
        } => {
            let previous_next_local_variable_location = *next_local_variable_location;
            let next_local_variable_location = &mut next_local_variable_location.clone();
            for &expression in expressions {
                compute_local_variable_locations(
                    expression,
                    nodes,
                    types,
                    next_local_variable_location,
                    local_variable_locations,
                );
            }
            let mut space_to_allocate =
                *next_local_variable_location - previous_next_local_variable_location;
            space_to_allocate += (space_to_allocate + STACK_ALIGNMENT - 1) % STACK_ALIGNMENT;
            writeln!(f, "    sub rsp, {}", space_to_allocate)?;
            let mut last_expression_emit_info = None;
            for &expression in expressions {
                last_expression_emit_info = Some(emit(
                    expression,
                    nodes,
                    types,
                    f,
                    next_local_variable_location,
                    local_variable_locations,
                )?);
            }
            writeln!(f, "    add rsp, {}", space_to_allocate)?;
            if let Some(emit_info) = last_expression_emit_info {
                emit_info
            } else {
                EmitInfo { l_value: false }
            }
        }
        BoundNode::Constant { ref value, .. } => match *value {
            Value::Type { typ } => {
                _ = typ;
                todo!()
            }
            Value::Procedure { procedure } => emit(
                procedure,
                nodes,
                types,
                f,
                next_local_variable_location,
                local_variable_locations,
            )?,
        },
        BoundNode::Declaration { typ, value, .. } => todo!(),
        BoundNode::Type { typ, type_type, .. } => todo!(),
        BoundNode::Name {
            referenced_node, ..
        } => todo!(),
        BoundNode::MemberAccess {
            operand,
            member_index,
            ..
        } => todo!(),
        BoundNode::Call {
            operand,
            ref arguments,
            ..
        } => todo!(),
        BoundNode::Cast {
            to_type,
            ref from_expressions,
            ..
        } => todo!(),
        BoundNode::Procedure { .. } => {
            writeln!(f, "    mov rax, _{}", node.into_inner())?;
            EmitInfo { l_value: false }
        }
    })
}

fn compute_local_variable_locations<'filepath>(
    node: NodeID<BoundNode<'filepath>>,
    nodes: &Nodes<BoundNode<'filepath>>,
    types: &Nodes<Type>,
    next_local_variable_location: &mut usize,
    local_variable_locations: &mut HashMap<NodeID<BoundNode<'filepath>>, usize>,
) {
}
