use derive_more::Display;

mod lexer;
mod source_location;

pub use lexer::*;
pub use source_location::*;

#[derive(Debug, Display, Clone, PartialEq)]
pub enum TokenKind<'source> {
    #[display(fmt = "{{newline}}")]
    Newline,
    #[display(fmt = "{_0}")]
    Name(&'source str),
    #[display(fmt = "->")]
    RightArrow,
    #[display(fmt = ":")]
    Colon,
    #[display(fmt = ";")]
    Semicolon,
    #[display(fmt = "+")]
    Plus,
    #[display(fmt = "-")]
    Minus,
    #[display(fmt = "*")]
    Asterisk,
    #[display(fmt = "/")]
    Slash,
    #[display(fmt = "^")]
    Caret,
    #[display(fmt = ".")]
    Period,
    #[display(fmt = ",")]
    Comma,
    #[display(fmt = "=")]
    Equal,
    #[display(fmt = "(")]
    OpenParenthesis,
    #[display(fmt = ")")]
    CloseParenthesis,
    #[display(fmt = "{{")]
    OpenBrace,
    #[display(fmt = "}}")]
    CloseBrace,
    #[display(fmt = "[")]
    OpenSquareBracket,
    #[display(fmt = "]")]
    CloseSquareBracket,
}

#[derive(Debug, Display, Clone, PartialEq)]
#[display(fmt = "{kind}")]
pub struct Token<'filepath, 'source> {
    pub kind: TokenKind<'source>,
    pub location: SourceLocation<'filepath>,
    pub end_location: SourceLocation<'filepath>,
}

impl<'filepath, 'source> GetLocation<'filepath> for Token<'filepath, 'source> {
    fn get_location(&self) -> SourceLocation<'filepath> {
        self.location
    }

    fn get_end_location(&self) -> SourceLocation<'filepath> {
        self.end_location
    }
}
