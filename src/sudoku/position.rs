/// Represents a position within a sudoku puzzle
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}
