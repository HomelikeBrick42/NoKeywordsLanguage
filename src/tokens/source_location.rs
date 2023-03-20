use std::num::NonZeroUsize;

use derive_more::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[display(fmt = "{filepath}:{line}:{column}")]
pub struct SourceLocation<'filepath> {
    pub filepath: &'filepath str,
    pub position: usize,
    pub line: NonZeroUsize,
    pub column: NonZeroUsize,
}

pub trait GetLocation<'filepath> {
    fn get_location(&self) -> SourceLocation<'filepath>;
}
