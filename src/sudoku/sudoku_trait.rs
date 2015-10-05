use std::ops::IndexMut;
use super::group::Group;
use super::position::Position;

/// Trait providing a generic interface for sudoku puzzles
pub trait Sudoku : IndexMut<Position, Output=char> {
    /// The allowed characters for the sudoku
    fn allowed_characters(&self) -> &'static str;

    /// The Positions in the row in which `pos` lies, excluding `pos` itself
    fn row_positions(&self, pos: Position) -> Group<Position>;

    /// The Positions in the column in which `pos` lies, excluding `pos` itself
    fn col_positions(&self, pos: Position) -> Group<Position>;

    /// The Positions in the box in which `pos` lies, excluding `pos` itself
    fn box_positions(&self, pos: Position) -> Group<Position>;

    /// The list of all positions which share a group with `pos`, excluding `pos` itself
    fn groups_positions(&self, pos: Position) -> Group<Position> {
        [self.row_positions(pos), self.col_positions(pos), self.box_positions(pos)]
            .into_iter().flat_map(|g| g.iter()).map(|p| *p).collect()
    }

    /// The elements contained in the row in which `pos` lies, excluding `pos`'s element
    fn row_elems(&self, pos: Position) -> Group<char> {
        self.row_positions(pos).iter().map(|&p| self[p]).filter(|&c| c!='.').collect()
    }

    /// The elements contained in the column in which `pos` lies, excluding `pos`'s element
    fn col_elems(&self, pos: Position) -> Group<char> {
        self.col_positions(pos).iter().map(|&p| self[p]).filter(|&c| c!='.').collect()
    }

    /// The elements contained in the box in which `pos` lies, excluding `pos`'s element
    fn box_elems(&self, pos: Position) -> Group<char> {
        self.box_positions(pos).iter().map(|&p| self[p]).filter(|&c| c!='.').collect()
    }

    /// The elements contained in any of the groups in which `pos` lies, excluding `pos`'s element
    fn groups_elems(&self, pos: Position) -> Group<char> {
        self.groups_positions(pos).iter().map(|&p| self[p]).filter(|&c| c!='.').collect()
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
        self[pos]!=element && self.groups_elems(pos).contains(element)
    }
}
