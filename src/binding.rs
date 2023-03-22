use std::collections::HashMap;

use crate::{
    nodes::{NodeID, Nodes},
    parsing::Ast,
    tokens::{GetLocation, SourceLocation},
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
            ref open_brace_token,
            ref expressions,
            ref close_brace_token,
        } => {
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
                location: open_brace_token.get_location(),
                end_location: close_brace_token.get_end_location(),
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
            _ = name_token;
            _ = typ;
            _ = value;
            todo!()
        }
        Ast::Declaration {
            ref name_token,
            ref typ,
            ref value,
            ..
        } => {
            _ = name_token;
            _ = typ;
            _ = value;
            todo!()
        }
        Ast::Name { ref name_token } => {
            _ = name_token;
            todo!()
        }
        Ast::ParenthesisedExpression { ref expression, .. } => {
            bind_expression(expression, nodes, types, names, common_types, type_hint)?
        }
        Ast::MemberAccess {
            ref operand,
            ref member_name_token,
            ..
        } => {
            _ = operand;
            _ = member_name_token;
            todo!()
        }
        Ast::Procedure {
            ref open_parenthesis_token,
            ref parameters,
            ref return_type,
            ref body,
            ..
        } => {
            _ = open_parenthesis_token;
            _ = parameters;
            _ = return_type;
            _ = body;
            todo!()
        }
        Ast::ProcedureType {
            ref open_parenthesis_token,
            ref parameters,
            ref return_type,
            ..
        } => {
            _ = open_parenthesis_token;
            _ = parameters;
            _ = return_type;
            todo!()
        }
        Ast::Call {
            ref operand,
            ref arguments,
            ref close_parenthesis_token,
            ..
        } => {
            _ = operand;
            _ = arguments;
            _ = close_parenthesis_token;
            todo!()
        }
        Ast::StructType {
            ref open_parenthesis_token,
            ref members,
            ref close_parenthesis_token,
        } => {
            _ = open_parenthesis_token;
            _ = members;
            _ = close_parenthesis_token;
            todo!();
        }
        Ast::SliceType {
            ref open_square_bracket_token,
            ref operand,
            ..
        } => {
            _ = open_square_bracket_token;
            _ = operand;
            todo!()
        }
        Ast::ArrayType {
            ref open_square_bracket_token,
            ref length,
            ref operand,
            ..
        } => {
            _ = open_square_bracket_token;
            _ = length;
            _ = operand;
            todo!()
        }
        Ast::MultipointerType {
            ref open_square_bracket_token,
            ref operand,
            ..
        } => {
            _ = open_square_bracket_token;
            _ = operand;
            todo!()
        }
    })
}

#[derive(Debug, Display)]
pub enum BindingError<'filepath> {
    #[display(fmt = "{_0}: Only constants are allowed in the global scope")]
    OnlyConstantsInGlobalScope(SourceLocation<'filepath>),
}
