#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Source {
    Data,
    Add,
}

#[derive(Debug)]
pub struct Piece {
    pub source: Source,
    pub offset: usize,
    pub length: usize,
}
impl Piece {
    pub fn new(action: Source, offset: usize, length: usize) -> Piece {
        Piece {
            source: action,
            offset,
            length,
        }
    }
}
