use std::{iter::Peekable, num::NonZeroUsize, str::CharIndices};

use derive_more::Display;

use super::{SourceLocation, Token};

pub struct Lexer<'filepath, 'source> {
    filepath: &'filepath str,
    source: Peekable<CharIndices<'source>>,
    position: usize,
    line: NonZeroUsize,
    column: NonZeroUsize,
}

impl<'filepath, 'source> Lexer<'filepath, 'source> {
    pub fn new(filepath: &'filepath str, source: &'source str) -> Self {
        Self {
            filepath,
            source: source.char_indices().peekable(),
            position: 0,
            line: 1.try_into().unwrap(),
            column: 1.try_into().unwrap(),
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        let (position, current) = self.source.next()?;
        self.position = position;
        self.column = self.line.checked_add(1).unwrap();
        if current == '\n' {
            self.line = self.line.checked_add(1).unwrap();
            self.column = 1.try_into().unwrap();
        }
        Some(current)
    }
}

impl<'filepath, 'source> Iterator for Lexer<'filepath, 'source> {
    type Item = Result<Token<'filepath, 'source>, LexerError<'filepath>>;

    fn next(&mut self) -> Option<Self::Item> {
        _ = self.filepath;
        _ = self.source;
        todo!()
    }
}

#[derive(Debug, Display)]
pub enum LexerError<'filepath> {
    #[display(fmt = "{location}: Unexpected character: {unexpected_character:?}")]
    UnexpectedCharacter {
        location: SourceLocation<'filepath>,
        unexpected_character: char,
    },
}
