use super::{SourceLocation, Token, TokenKind};
use derive_more::Display;
use std::{iter::Peekable, str::CharIndices};

#[derive(Debug, Clone)]
pub struct Lexer<'filepath, 'source> {
    location: SourceLocation<'filepath>,
    source: &'source str,
    chars: Peekable<CharIndices<'source>>,
}

impl<'filepath, 'source> Lexer<'filepath, 'source> {
    pub fn new(filepath: &'filepath str, source: &'source str) -> Self {
        Self {
            location: SourceLocation {
                filepath,
                position: 0,
                line: 1.try_into().unwrap(),
                column: 1.try_into().unwrap(),
            },
            source,
            chars: source.char_indices().peekable(),
        }
    }

    pub fn get_filepath(&self) -> &'filepath str {
        self.location.filepath
    }

    pub fn peek_char(&mut self) -> Option<char> {
        Some(self.chars.peek()?.1)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let (position, current) = self.chars.next()?;
        self.location.position = position + current.len_utf8();
        self.location.column = self.location.column.checked_add(1).unwrap();
        if current == '\n' {
            self.location.line = self.location.line.checked_add(1).unwrap();
            self.location.column = 1.try_into().unwrap();
        }
        Some(current)
    }

    pub fn peek(&self) -> Option<<Self as Iterator>::Item> {
        self.clone().next()
    }
}

impl<'filepath, 'source> Iterator for Lexer<'filepath, 'source> {
    type Item = Result<Token<'filepath, 'source>, LexerError<'filepath>>;

    fn next(&mut self) -> Option<Self::Item> {
        'main_loop: loop {
            let start_location = self.location;
            return Some(Ok(Token {
                kind: match self.next_char()? {
                    '\n' => TokenKind::Newline,
                    c if c.is_whitespace() => continue 'main_loop,

                    '-' if self.peek_char() == Some('>') => {
                        self.next_char();
                        TokenKind::RightArrow
                    }

                    ':' => TokenKind::Colon,
                    ';' => TokenKind::Semicolon,
                    '+' => TokenKind::Plus,
                    '-' => TokenKind::Minus,
                    '*' => TokenKind::Asterisk,
                    '/' => TokenKind::Slash,
                    '^' => TokenKind::Caret,
                    '.' => TokenKind::Period,
                    ',' => TokenKind::Comma,
                    '=' => TokenKind::Equal,
                    '(' => TokenKind::OpenParenthesis,
                    ')' => TokenKind::CloseParenthesis,
                    '{' => TokenKind::OpenBrace,
                    '}' => TokenKind::CloseBrace,
                    '[' => TokenKind::OpenSquareBracket,
                    ']' => TokenKind::CloseSquareBracket,

                    c if c.is_alphabetic() || c == '_' => {
                        loop {
                            let Some(c) = self.peek_char() else { break };
                            if c.is_alphanumeric() || c == '_' {
                                self.next_char();
                            } else {
                                break;
                            }
                        }

                        TokenKind::Name(
                            &self.source[start_location.position..self.location.position],
                        )
                    }

                    c => {
                        return Some(Err(LexerError::UnexpectedCharacter {
                            location: start_location,
                            unexpected_character: c,
                        }));
                    }
                },
                location: start_location,
                end_location: self.location,
            }));
        }
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
