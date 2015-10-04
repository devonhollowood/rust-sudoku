/// Represents a position within a sudoku puzzle
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    row: usize,
    col: usize,
}
