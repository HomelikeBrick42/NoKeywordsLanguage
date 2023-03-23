use std::collections::HashMap;

use crate::{
    eval::{eval_bound_node, Value},
    nodes::{NodeID, Nodes},
    parsing::Ast,
    tokens::{GetLocation, SourceLocation, TokenKind},
};
use derive_more::Display;

mod bound_nodes;
mod common_types;
mod types;

pub use bound_nodes::*;
pub use common_types::*;
pub use types::*;

pub fn bind_file<'filepath, 'source>(
    filepath: &'filepath str,
    expressions: &[Ast<'filepath, 'source>],
    nodes: &mut Nodes<BoundNode<'filepath>>,
    types: &mut Nodes<Type>,
    names: &mut HashMap<&'source str, NodeID<BoundNode<'filepath>>>,
    common_types: &mut CommonTypes,
) -> Result<NodeID<BoundNode<'filepath>>, BindingError<'filepath>> {
    let bound_expressions = expressions
        .iter()
        .map(|expression| {
            if !matches!(expression, Ast::Constant { .. }) {
                return Err(BindingError::OnlyConstantsInGlobalScope(
                    expression.get_location(),
                ));
            }
            bind_expression(expression, nodes, types, names, common_types, None)
        })
        .collect::<Result<_, _>>()?;

    let file_location = SourceLocation {
        filepath,
        position: 0,
        line: 1.try_into().unwrap(),
        column: 1.try_into().unwrap(),
    };
    Ok(nodes.insert(BoundNode::Block {
        location: file_location,
        end_location: file_location,
        expressions: bound_expressions,
        result_type: common_types.void,
    }))
}

fn bind_expression<'filepath, 'source>(
    expression: &Ast<'filepath, 'source>,
    nodes: &mut Nodes<BoundNode<'filepath>>,
    types: &mut Nodes<Type>,
    names: &mut HashMap<&'source str, NodeID<BoundNode<'filepath>>>,
    common_types: &mut CommonTypes,
    type_hint: Option<NodeID<Type>>,
) -> Result<NodeID<BoundNode<'filepath>>, BindingError<'filepath>> {
    Ok(match *expression {
        Ast::Block {
            ref expressions, ..
        } => {
            let names = &mut names.clone();

            let bound_expressions = expressions
                .iter()
                .map(|expression| {
                    bind_expression(expression, nodes, types, names, common_types, None)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let result_type = bound_expressions.last().map_or_else(
                || {
                    if let Some(hint) = type_hint {
                        if matches!(types[hint], Type::Void) {
                            return hint;
                        }
                    }
                    common_types.void
                },
                |&expression| nodes[expression].get_type(nodes),
            );

            nodes.insert(BoundNode::Block {
                location: expression.get_location(),
                end_location: expression.get_end_location(),
                expressions: bound_expressions,
                result_type,
            })
        }
        Ast::Constant {
            ref name_token,
            ref typ,
            ref value,
            ..
        } => {
            let typ = typ
                .as_ref()
                .map(|typ| {
                    let typ = bind_expression(
                        typ,
                        nodes,
                        types,
                        &mut names.clone(),
                        common_types,
                        Some(common_types.typ),
                    )?;

                    if !nodes[typ].is_constant(nodes) {
                        todo!()
                    }

                    if !matches!(types[nodes[typ].get_type(nodes)], Type::Type) {
                        todo!()
                    }

                    let Value::Type { typ } = eval_bound_node(typ, nodes) else { unreachable!() };
                    Ok(typ)
                })
                .transpose()?;

            let value = bind_expression(
                value,
                nodes,
                types,
                &mut names.clone(),
                common_types,
                typ.or(type_hint),
            )?;

            if !nodes[value].is_constant(nodes) {
                todo!()
            }

            let value_type = nodes[value].get_type(nodes);

            if let Some(typ) = typ {
                if value_type != typ {
                    todo!()
                }
            }

            let typ = typ.unwrap_or(value_type);

            let value = eval_bound_node(value, nodes);

            let constant = nodes.insert(BoundNode::Constant {
                location: expression.get_location(),
                end_location: expression.get_end_location(),
                typ,
                value,
            });

            let TokenKind::Name(name) = name_token.kind else { unreachable!() };
            names.insert(name, constant);

            constant
        }
        Ast::Declaration {
            ref name_token,
            ref typ,
            ref value,
            ..
        } => {
            let typ = typ
                .as_ref()
                .map(|typ| {
                    let typ = bind_expression(
                        typ,
                        nodes,
                        types,
                        &mut names.clone(),
                        common_types,
                        Some(common_types.typ),
                    )?;

                    if !nodes[typ].is_constant(nodes) {
                        todo!()
                    }

                    if !matches!(types[nodes[typ].get_type(nodes)], Type::Type) {
                        todo!()
                    }

                    let Value::Type { typ } = eval_bound_node(typ, nodes) else { unreachable!() };
                    Ok(typ)
                })
                .transpose()?;

            let value = value
                .as_ref()
                .map(|value| {
                    let value = bind_expression(
                        value,
                        nodes,
                        types,
                        &mut names.clone(),
                        common_types,
                        typ,
                    )?;
                    Ok(value)
                })
                .transpose()?;

            let typ = typ.unwrap_or_else(|| nodes[value.unwrap()].get_type(nodes));

            let declaration = nodes.insert(BoundNode::Declaration {
                location: expression.get_location(),
                end_location: expression.get_end_location(),
                typ,
                value,
            });

            let TokenKind::Name(name) = name_token.kind else { unreachable!() };
            names.insert(name, declaration);

            declaration
        }
        Ast::Name { ref name_token } => {
            let TokenKind::Name(name) = name_token.kind else { unreachable!() };
            let Some(&node) = names.get(name) else {
                todo!()
            };
            nodes.insert(BoundNode::Name {
                location: expression.get_location(),
                end_location: expression.get_end_location(),
                referenced_node: node,
            })
        }
        Ast::ParenthesisedExpression { ref expression, .. } => bind_expression(
            expression,
            nodes,
            types,
            &mut names.clone(),
            common_types,
            type_hint,
        )?,
        Ast::MemberAccess {
            ref operand,
            ref member_name_token,
            ..
        } => {
            let operand = bind_expression(operand, nodes, types, names, common_types, None)?;
            let operand_type = nodes[operand].get_type(nodes);
            let TokenKind::Name(name) = member_name_token.kind else { unreachable!() };
            let (member_index, result_type) = match types[operand_type] {
                Type::Slice { inner_type } => match name {
                    "data" => (0, common_types.get_pointer(types, inner_type)),
                    "length" => (1, common_types.uint),
                    _ => todo!(),
                },
                _ => todo!(),
            };
            nodes.insert(BoundNode::MemberAccess {
                location: expression.get_location(),
                end_location: expression.get_end_location(),
                operand,
                member_index,
                result_type,
            })
        }
        Ast::Procedure {
            ref parameters,
            ref return_type,
            ref body,
            ..
        } => {
            let names = &mut names.clone();

            let parameters = parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| {
                    let type_hint = type_hint.and_then(|typ| {
                        if let Type::Procedure { ref parameters, .. } = types[typ] {
                            parameters.get(i).copied()
                        } else {
                            None
                        }
                    });

                    let parameter =
                        bind_expression(parameter, nodes, types, names, common_types, type_hint)?;

                    if let BoundNode::Declaration { value, .. } = nodes[parameter] {
                        if value.is_some() {
                            todo!() // cannot have default procedure parameter values (for now)
                        }
                    } else {
                        unreachable!()
                    }

                    Ok(parameter)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let return_type = bind_expression(
                return_type,
                nodes,
                types,
                &mut names.clone(),
                common_types,
                Some(common_types.typ),
            )?;

            if !nodes[return_type].is_constant(nodes) {
                todo!()
            }

            if !matches!(types[nodes[return_type].get_type(nodes)], Type::Type) {
                todo!()
            }

            let Value::Type { typ: return_type } = eval_bound_node(return_type, nodes) else { unreachable!() };
            let body = bind_expression(body, nodes, types, names, common_types, Some(return_type))?;

            if nodes[body].get_type(nodes) != return_type {
                todo!()
            }

            let parameter_types = parameters
                .iter()
                .map(|&parameter| nodes[parameter].get_type(nodes))
                .collect::<Vec<_>>();

            let typ = type_hint
                .and_then(|typ| {
                    if let Type::Procedure {
                        parameters: ref other_parameters,
                        return_type: other_return_type,
                    } = types[typ]
                    {
                        (other_return_type == return_type && other_parameters == &parameter_types)
                            .then_some(typ)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| {
                    common_types.get_procedure(types, &parameter_types, return_type)
                });

            nodes.insert(BoundNode::Procedure {
                location: expression.get_location(),
                end_location: expression.get_end_location(),
                parameters,
                return_type,
                typ,
                body,
            })
        }
        Ast::ProcedureType {
            ref parameters,
            ref return_type,
            ..
        } => {
            _ = parameters;
            _ = return_type;
            todo!()
        }
        Ast::Call {
            ref operand,
            ref arguments,
            ..
        } => {
            let operand = bind_expression(operand, nodes, types, names, common_types, None)?;
            let operand_type = nodes[operand].get_type(nodes);
            match types[operand_type] {
                Type::Type => {
                    if !nodes[operand].is_constant(nodes) {
                        todo!()
                    }
                    if !matches!(types[nodes[operand].get_type(nodes)], Type::Type) {
                        todo!()
                    }
                    let Value::Type { typ: to_type } = eval_bound_node(operand, nodes) else { unreachable!() };

                    let arguments = arguments
                        .iter()
                        .enumerate()
                        .map(|(i, argument)| {
                            let type_hint = match (&types[to_type], i, arguments.len()) {
                                (_, 0, 1) => Some(to_type),
                                (_, _, _) => None,
                            };

                            bind_expression(argument, nodes, types, names, common_types, type_hint)
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    let argument_types = arguments
                        .iter()
                        .map(|&argument| nodes[argument].get_type(nodes))
                        .collect::<Vec<_>>();

                    if argument_types.len() == 1 && to_type == argument_types[0] {
                        arguments[0]
                    } else if argument_types.len() == 1
                        && matches!(
                            (&types[to_type], &types[argument_types[0]]),
                            (Type::Int, Type::UInt)
                        )
                    {
                        nodes.insert(BoundNode::Cast {
                            location: expression.get_location(),
                            end_location: expression.get_end_location(),
                            to_type,
                            from_expressions: arguments,
                        })
                    } else {
                        todo!()
                    }
                }
                Type::Procedure {
                    ref parameters,
                    return_type,
                } => {
                    _ = parameters;
                    _ = return_type;
                    todo!()
                }
                _ => {
                    todo!()
                }
            }
        }
        Ast::StructType { ref members, .. } => {
            _ = members;
            todo!();
        }
        Ast::SliceType { ref operand, .. } => {
            let operand = bind_expression(
                operand,
                nodes,
                types,
                &mut names.clone(),
                common_types,
                Some(common_types.typ),
            )?;

            if !nodes[operand].is_constant(nodes) {
                todo!()
            }

            if !matches!(types[nodes[operand].get_type(nodes)], Type::Type) {
                todo!()
            }

            let Value::Type { typ: operand } = eval_bound_node(operand, nodes) else { unreachable!() };

            let typ = common_types.get_slice(types, operand);

            nodes.insert(BoundNode::Type {
                location: expression.get_location(),
                end_location: expression.get_end_location(),
                typ,
                type_type: type_hint
                    .and_then(|typ| matches!(types[typ], Type::Type).then_some(typ))
                    .unwrap_or(common_types.typ),
            })
        }
        Ast::ArrayType {
            ref length,
            ref operand,
            ..
        } => {
            _ = length;
            _ = operand;
            todo!()
        }
        Ast::MultipointerType { ref operand, .. } => {
            let operand = bind_expression(
                operand,
                nodes,
                types,
                &mut names.clone(),
                common_types,
                Some(common_types.typ),
            )?;

            if !nodes[operand].is_constant(nodes) {
                todo!()
            }

            if !matches!(types[nodes[operand].get_type(nodes)], Type::Type) {
                todo!()
            }

            let Value::Type { typ: operand } = eval_bound_node(operand, nodes) else { unreachable!() };

            let typ = common_types.get_multipointer(types, operand);

            nodes.insert(BoundNode::Type {
                location: expression.get_location(),
                end_location: expression.get_end_location(),
                typ,
                type_type: type_hint
                    .and_then(|typ| matches!(types[typ], Type::Type).then_some(typ))
                    .unwrap_or(common_types.typ),
            })
        }
    })
}

#[derive(Debug, Display)]
pub enum BindingError<'filepath> {
    #[display(fmt = "{_0}: Only constants are allowed in the global scope")]
    OnlyConstantsInGlobalScope(SourceLocation<'filepath>),
}
