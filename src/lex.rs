#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Loc {
    pub pos: usize,
    pub row: usize,
    pub col: usize,
}

impl Loc {
    pub fn new(pos: usize, row: usize, col: usize) -> Self {
        Loc {
            pos,
            row,
            col,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span<'a> {
    pub file_name: &'a str,
    pub start: Loc,
    pub end: Loc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tok<'a> {
    pub type: Tt,
    pub span: Span<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tt {
}

#[derive(Debug)]
pub struct Lexer<'a, 'b> {
    file_name: &'a str,
    input: &'b str,
}

impl<'a, 'b> Lexer<'a, 'b> {
    pub fn new(file_name: &'a str, input: &'b str) -> Self {
        Lexer {
            file_name,
            input,
        }
    }
}
