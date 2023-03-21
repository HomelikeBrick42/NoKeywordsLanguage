use crate::tokens::{GetLocation, SourceLocation, Token};

#[derive(Debug, Clone)]
pub enum Ast<'filepath, 'source> {
    Block {
        open_brace_token: Token<'filepath, 'source>,
        expressions: Vec<Ast<'filepath, 'source>>,
        close_brace_token: Token<'filepath, 'source>,
    },
    Constant {
        name_token: Token<'filepath, 'source>,
        colon_token: Token<'filepath, 'source>,
        typ: Option<Box<Ast<'filepath, 'source>>>,
        colon_equals_token: Token<'filepath, 'source>,
        value: Box<Ast<'filepath, 'source>>,
    },
    Declaration {
        name_token: Token<'filepath, 'source>,
        colon_token: Token<'filepath, 'source>,
        typ: Option<Box<Ast<'filepath, 'source>>>,
        equals_token: Option<Token<'filepath, 'source>>,
        value: Option<Box<Ast<'filepath, 'source>>>,
    },
    Name {
        name_token: Token<'filepath, 'source>,
    },
    ParenthesisedExpression {
        open_parenthesis_token: Token<'filepath, 'source>,
        expression: Box<Ast<'filepath, 'source>>,
        close_parenthesis_token: Token<'filepath, 'source>,
    },
    MemberAccess {
        operand: Box<Ast<'filepath, 'source>>,
        period_token: Token<'filepath, 'source>,
        member_name_token: Token<'filepath, 'source>,
    },
    Procedure {
        open_parenthesis_token: Token<'filepath, 'source>,
        parameters: Vec<Ast<'filepath, 'source>>,
        close_parenthesis_token: Token<'filepath, 'source>,
        right_arrow_token: Token<'filepath, 'source>,
        return_type: Box<Ast<'filepath, 'source>>,
        body: Box<Ast<'filepath, 'source>>,
    },
    ProcedureType {
        open_parenthesis_token: Token<'filepath, 'source>,
        parameters: Vec<Ast<'filepath, 'source>>,
        close_parenthesis_token: Token<'filepath, 'source>,
        right_arrow_token: Token<'filepath, 'source>,
        return_type: Box<Ast<'filepath, 'source>>,
    },
    Call {
        operand: Box<Ast<'filepath, 'source>>,
        open_parenthesis_token: Token<'filepath, 'source>,
        arguments: Vec<Ast<'filepath, 'source>>,
        close_parenthesis_token: Token<'filepath, 'source>,
    },
    StructType {
        open_parenthesis_token: Token<'filepath, 'source>,
        members: Vec<Ast<'filepath, 'source>>,
        close_parenthesis_token: Token<'filepath, 'source>,
    },
    SliceType {
        open_square_bracket_token: Token<'filepath, 'source>,
        close_square_bracket_token: Token<'filepath, 'source>,
        operand: Box<Ast<'filepath, 'source>>,
    },
    ArrayType {
        open_square_bracket_token: Token<'filepath, 'source>,
        length: Box<Ast<'filepath, 'source>>,
        close_square_bracket_token: Token<'filepath, 'source>,
        operand: Box<Ast<'filepath, 'source>>,
    },
    MultipointerType {
        open_square_bracket_token: Token<'filepath, 'source>,
        caret_token: Token<'filepath, 'source>,
        close_square_bracket_token: Token<'filepath, 'source>,
        operand: Box<Ast<'filepath, 'source>>,
    },
}

impl<'filepath, 'source> GetLocation<'filepath> for Ast<'filepath, 'source> {
    fn get_location(&self) -> SourceLocation<'filepath> {
        match *self {
            Ast::Block {
                ref open_brace_token,
                ..
            } => open_brace_token.get_location(),
            Ast::Constant { ref name_token, .. } => name_token.get_location(),
            Ast::Declaration { ref name_token, .. } => name_token.get_location(),
            Ast::Name { ref name_token } => name_token.get_location(),
            Self::ParenthesisedExpression {
                ref open_parenthesis_token,
                ..
            } => open_parenthesis_token.get_location(),
            Ast::MemberAccess { ref operand, .. } => operand.get_location(),
            Ast::Procedure {
                ref open_parenthesis_token,
                ..
            } => open_parenthesis_token.get_location(),
            Ast::ProcedureType {
                ref open_parenthesis_token,
                ..
            } => open_parenthesis_token.get_location(),
            Ast::Call { ref operand, .. } => operand.get_location(),
            Ast::StructType {
                ref open_parenthesis_token,
                ..
            } => open_parenthesis_token.get_location(),
            Ast::SliceType {
                ref open_square_bracket_token,
                ..
            } => open_square_bracket_token.get_location(),
            Ast::ArrayType {
                ref open_square_bracket_token,
                ..
            } => open_square_bracket_token.get_location(),
            Ast::MultipointerType {
                ref open_square_bracket_token,
                ..
            } => open_square_bracket_token.get_location(),
        }
    }

    fn get_end_location(&self) -> SourceLocation<'filepath> {
        match *self {
            Ast::Block {
                ref close_brace_token,
                ..
            } => close_brace_token.get_end_location(),
            Ast::Constant { ref value, .. } => value.get_end_location(),
            Ast::Declaration {
                ref typ, ref value, ..
            } => {
                // there should be at least a type or a value
                if let Some(value) = value {
                    value.get_end_location()
                } else {
                    typ.as_ref().unwrap().get_end_location()
                }
            }
            Ast::Name { ref name_token } => name_token.get_end_location(),
            Ast::ParenthesisedExpression {
                ref close_parenthesis_token,
                ..
            } => close_parenthesis_token.get_end_location(),
            Ast::MemberAccess {
                ref member_name_token,
                ..
            } => member_name_token.get_end_location(),
            Ast::Procedure { ref body, .. } => body.get_end_location(),
            Ast::ProcedureType {
                ref return_type, ..
            } => return_type.get_end_location(),
            Ast::Call {
                ref close_parenthesis_token,
                ..
            } => close_parenthesis_token.get_end_location(),
            Ast::StructType {
                ref close_parenthesis_token,
                ..
            } => close_parenthesis_token.get_end_location(),
            Ast::SliceType { ref operand, .. } => operand.get_end_location(),
            Ast::ArrayType { ref operand, .. } => operand.get_end_location(),
            Ast::MultipointerType { ref operand, .. } => operand.get_end_location(),
        }
    }
}
