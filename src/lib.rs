use std::ops::Range;

pub mod terminal;
pub mod editor;
pub mod buffer;


#[derive(Default, Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
    pub fn get_terminal(&self) -> (u16, u16) {
        assert!(self.x < u16::MAX as usize && self.y < u16::MAX as usize);
        (self.x as u16 + 1, self.y as u16 + 1)
    }
}


/// Splits a range at indices if possible. Assumes indices are sorted.
/// Example: range=0..10, indices=[2, 5] => Result=[0..2, 2..5, 5..10]
pub fn split_range(range: &Range<usize>, indices: &[usize]) -> Vec<Range<usize>> {
    let mut ranges = vec![];
    let mut start = range.start;

    for idx in indices {
        if !range.contains(idx) {
            break;
        }

        ranges.push(start..*idx);
        start = *idx;
    }

    ranges.push(start..range.end);
   
    if ranges.is_empty() {
        ranges.push(range.clone());
    }

    ranges
}
