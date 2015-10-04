use std::ops::IndexMut;
use super::group::Group;

/// Represents a position within a sudoku puzzle
pub type Position = (usize, usize);

/// Trait providing a generic interface for sudoku puzzles
pub trait Sudoku : IndexMut<Position, Output=char> {
    /// The row in which `pos` lies
    fn get_row(&self, pos: Position) -> Group;

    /// The col in which `pos` lies
    fn get_col(&self, pos: Position) -> Group;

    /// The box in which `pos` lies
    fn get_box(&self, pos: Position) -> Group;

    /// The allowed characters for the sudoku
    fn allowed_characters(&self) -> &'static str;

    /// The list of groups in which `pos` lies
    fn groups(&self, pos: Position) -> [Group; 3] {
        [self.get_row(pos), self.get_col(pos), self.get_box(pos)]
    }

    /// The list of characters which `pos` could be
    fn possibilities(&self, pos: Position) -> Vec<char> {
        if self[pos] == '.' {
            self.allowed_characters().chars().filter(|&c| !self.conflicts(pos, c)).collect()
        }
        else {
            vec![self[pos]]
        }
    }

    /// True if `element` is located in one of `pos`'s groups (besides the position of `pos`
    /// itself)
    fn conflicts(&self, pos: Position, element: char) -> bool {
        self[pos]!=element && self.groups(pos).iter().any(|g| g.contains(element))
    }
}
