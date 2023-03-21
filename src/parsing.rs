use crate::tokens::{GetLocation, Lexer, LexerError, SourceLocation, Token, TokenKind};
use derive_more::Display;

mod ast;

pub use ast::*;

pub fn parse_file<'filepath, 'source>(
    filepath: &'filepath str,
    source: &'source str,
) -> Result<Vec<Ast<'filepath, 'source>>, ParsingError<'filepath, 'source>> {
    let mut lexer = Lexer::new(filepath, source);
    let mut expressions = vec![];
    while lexer.peek().is_some() {
        while let Some(_newline) = match_token(&mut lexer, TokenKind::Newline)? {}
        if lexer.peek().is_none() {
            break;
        }
        expressions.push(parse_expression(&mut lexer)?);
        expect_newline(&mut lexer)?;
    }
    Ok(expressions)
}

fn parse_expression<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
) -> Result<Ast<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    let mut expression = match next_token(lexer)? {
        open_parenthesis_token @ Token {
            kind: TokenKind::OpenParenthesis,
            ..
        } => {
            let expression = parse_expression(lexer)?;
            if match_token(lexer, TokenKind::Comma)?.is_some() {
                if !matches!(expression, Ast::Declaration { .. }) {
                    return Err(ParsingError::ExpectedDeclarationForProcedureOrStruct {
                        location: expression.get_location(),
                    });
                }

                let mut declarations = vec![expression];

                while !matches!(
                    lexer.peek().transpose()?,
                    Some(Token {
                        kind: TokenKind::CloseParenthesis,
                        ..
                    })
                ) {
                    let name_token = next_token(lexer)?;
                    if !matches!(name_token.kind, TokenKind::Name(_)) {
                        return Err(ParsingError::ExpectedNameToken { got: name_token });
                    }
                    let colon_token = expect_token(lexer, TokenKind::Colon)?;

                    let typ = if !matches!(
                        lexer.peek().transpose()?,
                        Some(Token {
                            kind: TokenKind::Equal,
                            ..
                        })
                    ) {
                        Some(parse_expression(lexer)?)
                    } else {
                        None
                    };

                    let equals_token = match_token(lexer, TokenKind::Equal)?;
                    let value = if equals_token.is_some() {
                        Some(parse_expression(lexer)?)
                    } else {
                        None
                    };

                    declarations.push(Ast::Declaration {
                        name_token,
                        colon_token,
                        typ: typ.map(Box::new),
                        equals_token,
                        value: value.map(Box::new),
                    });

                    if matches!(
                        lexer.peek().transpose()?,
                        Some(Token {
                            kind: TokenKind::CloseParenthesis,
                            ..
                        })
                    ) {
                        break;
                    }

                    expect_comma(lexer)?;
                }

                let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;

                if let Some(right_arrow_token) = match_token(lexer, TokenKind::RightArrow)? {
                    let return_type = parse_expression(lexer)?;
                    if let Some(open_brace_token) = match_token(lexer, TokenKind::OpenBrace)? {
                        let body = parse_block(lexer, open_brace_token)?;
                        Ast::Procedure {
                            open_parenthesis_token,
                            parameters: declarations,
                            close_parenthesis_token,
                            right_arrow_token,
                            return_type: Box::new(return_type),
                            body: Box::new(body),
                        }
                    } else {
                        Ast::ProcedureType {
                            open_parenthesis_token,
                            parameters: declarations,
                            close_parenthesis_token,
                            right_arrow_token,
                            return_type: Box::new(return_type),
                        }
                    }
                } else {
                    Ast::StructType {
                        open_parenthesis_token,
                        members: declarations,
                        close_parenthesis_token,
                    }
                }
            } else {
                let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;
                if let Some(right_arrow_token) = match_token(lexer, TokenKind::RightArrow)? {
                    if !matches!(expression, Ast::Declaration { .. }) {
                        return Err(ParsingError::ExpectedDeclarationForProcedure {
                            location: expression.get_location(),
                        });
                    }

                    let parameters = vec![expression];

                    let return_type = parse_expression(lexer)?;
                    if let Some(open_brace_token) = match_token(lexer, TokenKind::OpenBrace)? {
                        let body = parse_block(lexer, open_brace_token)?;
                        Ast::Procedure {
                            open_parenthesis_token,
                            parameters,
                            close_parenthesis_token,
                            right_arrow_token,
                            return_type: Box::new(return_type),
                            body: Box::new(body),
                        }
                    } else {
                        Ast::ProcedureType {
                            open_parenthesis_token,
                            parameters,
                            close_parenthesis_token,
                            right_arrow_token,
                            return_type: Box::new(return_type),
                        }
                    }
                } else {
                    Ast::ParenthesisedExpression {
                        open_parenthesis_token,
                        expression: Box::new(expression),
                        close_parenthesis_token,
                    }
                }
            }
        }

        name_token @ Token {
            kind: TokenKind::Name(_),
            ..
        } => {
            if let Some(colon_token) = match_token(lexer, TokenKind::Colon)? {
                let typ = if !matches!(
                    lexer.peek().transpose()?,
                    Some(Token {
                        kind: TokenKind::Colon | TokenKind::Equal,
                        ..
                    })
                ) {
                    Some(parse_expression(lexer)?)
                } else {
                    None
                };

                if let Some(colon_equals_token) = match_token(lexer, TokenKind::Colon)? {
                    let value = parse_expression(lexer)?;
                    Ast::Constant {
                        name_token,
                        colon_token,
                        typ: typ.map(Box::new),
                        colon_equals_token,
                        value: Box::new(value),
                    }
                } else {
                    let equals_token = match_token(lexer, TokenKind::Equal)?;
                    let value = if equals_token.is_some() {
                        Some(parse_expression(lexer)?)
                    } else {
                        None
                    };
                    Ast::Declaration {
                        name_token,
                        colon_token,
                        typ: typ.map(Box::new),
                        equals_token,
                        value: value.map(Box::new),
                    }
                }
            } else {
                Ast::Name { name_token }
            }
        }

        open_square_bracket_token @ Token {
            kind: TokenKind::OpenSquareBracket,
            ..
        } => {
            if let Some(close_square_bracket_token) =
                match_token(lexer, TokenKind::CloseSquareBracket)?
            {
                let operand = parse_expression(lexer)?;
                Ast::SliceType {
                    open_square_bracket_token,
                    close_square_bracket_token,
                    operand: Box::new(operand),
                }
            } else if let Some(caret_token) = match_token(lexer, TokenKind::Caret)? {
                let close_square_bracket_token =
                    expect_token(lexer, TokenKind::CloseSquareBracket)?;
                let operand = parse_expression(lexer)?;
                Ast::MultipointerType {
                    open_square_bracket_token,
                    caret_token,
                    close_square_bracket_token,
                    operand: Box::new(operand),
                }
            } else {
                let length = parse_expression(lexer)?;
                let close_square_bracket_token =
                    expect_token(lexer, TokenKind::CloseSquareBracket)?;
                let operand = parse_expression(lexer)?;
                Ast::ArrayType {
                    open_square_bracket_token,
                    length: Box::new(length),
                    close_square_bracket_token,
                    operand: Box::new(operand),
                }
            }
        }

        token => return Err(ParsingError::UnexpectedToken(token)),
    };

    loop {
        expression =
            if let Some(open_parenthesis_token) = match_token(lexer, TokenKind::OpenParenthesis)? {
                let mut arguments = vec![];
                while !matches!(
                    lexer.peek().transpose()?,
                    Some(Token {
                        kind: TokenKind::CloseParenthesis,
                        ..
                    })
                ) {
                    arguments.push(parse_expression(lexer)?);
                    expect_comma(lexer)?;
                }
                let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;
                Ast::Call {
                    operand: Box::new(expression),
                    open_parenthesis_token,
                    arguments,
                    close_parenthesis_token,
                }
            } else if let Some(period_token) = match_token(lexer, TokenKind::Period)? {
                let member_name_token = next_token(lexer)?;
                if !matches!(member_name_token.kind, TokenKind::Name(_)) {
                    return Err(ParsingError::ExpectedNameToken {
                        got: member_name_token,
                    });
                }
                Ast::MemberAccess {
                    operand: Box::new(expression),
                    period_token,
                    member_name_token,
                }
            } else {
                break;
            };
    }

    Ok(expression)
}

fn parse_block<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
    open_brace_token: Token<'filepath, 'source>,
) -> Result<Ast<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    let mut expressions = vec![];
    while !matches!(
        lexer.peek().transpose()?,
        Some(Token {
            kind: TokenKind::CloseBrace,
            ..
        })
    ) {
        while let Some(_newline) = match_token(lexer, TokenKind::Newline)? {}
        if matches!(
            lexer.peek().transpose()?,
            Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            })
        ) {
            break;
        }
        expressions.push(parse_expression(lexer)?);
        expect_newline(lexer)?;
    }
    let close_brace_token = expect_token(lexer, TokenKind::CloseBrace)?;
    Ok(Ast::Block {
        open_brace_token,
        expressions,
        close_brace_token,
    })
}

fn next_token<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
) -> Result<Token<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    lexer
        .next()
        .transpose()?
        .ok_or_else(|| ParsingError::UnexpectedEOF(lexer.get_filepath()))
}

fn match_token<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
    kind: TokenKind<'source>,
) -> Result<Option<Token<'filepath, 'source>>, ParsingError<'filepath, 'source>> {
    lexer
        .peek()
        .transpose()?
        .filter(|token| token.kind == kind)
        .map(|_| next_token(lexer))
        .transpose()
}

// fn match_token_with<'filepath, 'source, F>(
//     lexer: &mut Lexer<'filepath, 'source>,
//     f: F,
// ) -> Result<Option<Token<'filepath, 'source>>, ParsingError<'filepath, 'source>>
// where
//     F: FnOnce(&'_ TokenKind<'source>) -> bool,
// {
//     lexer
//         .peek()
//         .transpose()?
//         .filter(|token| f(&token.kind))
//         .map(|_| next_token(lexer))
//         .transpose()
// }

fn expect_token<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
    kind: TokenKind<'source>,
) -> Result<Token<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    let token = next_token(lexer)?;
    if token.kind == kind {
        Ok(token)
    } else {
        Err(ParsingError::ExpectedToken {
            expected: kind,
            got: token,
        })
    }
}

fn expect_newline<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
) -> Result<(), ParsingError<'filepath, 'source>> {
    if let Some(token) = lexer.peek().transpose()? {
        match token.kind {
            TokenKind::Newline => {
                next_token(lexer)?;
                Ok(())
            }
            TokenKind::CloseParenthesis | TokenKind::CloseBrace | TokenKind::CloseSquareBracket => {
                Ok(())
            }
            _ => Err(ParsingError::ExpectedToken {
                expected: TokenKind::Newline,
                got: next_token(lexer)?,
            }),
        }
    } else {
        Ok(())
    }
}

fn expect_comma<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
) -> Result<(), ParsingError<'filepath, 'source>> {
    if let Some(token) = lexer.peek().transpose()? {
        match token.kind {
            TokenKind::Comma => {
                next_token(lexer)?;
                match_token(lexer, TokenKind::Newline)?;
                Ok(())
            }
            TokenKind::CloseParenthesis => Ok(()),
            _ => Err(ParsingError::ExpectedToken {
                expected: TokenKind::Comma,
                got: next_token(lexer)?,
            }),
        }
    } else {
        Ok(())
    }
}

#[derive(Debug, Display)]
pub enum ParsingError<'filepath, 'source> {
    #[display(fmt = "{_0}")]
    LexerError(LexerError<'filepath>),
    #[display(fmt = "Unexpected EOF in {_0}")]
    UnexpectedEOF(&'filepath str),
    #[display(fmt = "{}: Unexpected token {_0}", "_0.get_location()")]
    UnexpectedToken(Token<'filepath, 'source>),
    #[display(
        fmt = "{}: Expected token {expected}, but got {got}",
        "got.get_location()"
    )]
    ExpectedToken {
        expected: TokenKind<'source>,
        got: Token<'filepath, 'source>,
    },
    #[display(fmt = "{}: Expected name token, but got {got}", "got.get_location()")]
    ExpectedNameToken { got: Token<'filepath, 'source> },
    #[display(fmt = "{location}: Expected a declaration for procedure parameter")]
    ExpectedDeclarationForProcedure { location: SourceLocation<'filepath> },
    #[display(fmt = "{location}: Expected a declaration for procedure parameter or struct field")]
    ExpectedDeclarationForProcedureOrStruct { location: SourceLocation<'filepath> },
}

impl<'filepath, 'source> From<LexerError<'filepath>> for ParsingError<'filepath, 'source> {
    fn from(error: LexerError<'filepath>) -> Self {
        ParsingError::LexerError(error)
    }
}
