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
