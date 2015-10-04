//! Dynamically-sized sudoku

use std::ops::{Index, IndexMut};
use std::iter::IntoIterator;
use super::super::position::Position;

/// An implementation of a sudoku puzzle which is dynamic in size, depending on its input
pub struct Dynamic {
    data: Vec<char>,
    nrows: usize,
    ncols: usize,
    box_width: usize,
    box_height: usize,
}

impl Dynamic {
    /// Builds Dynamic from rows.
    pub fn from_rows(rows: &Vec<Vec<char>>) -> Dynamic
    {
        let data = rows.iter().flat_map(|r| r.iter().map(|c| *c)).collect();
        let side_len = rows.len();
        assert!(rows.len() > 0, "Must have at least one row!");
        assert!(rows.len() == rows[0].len(), "Puzzle must be square!");
        let box_width = match side_len {
            4  => 2,
            6  => 3,
            9  => 3,
            16 => 4,
            25 => 5,
            _  => panic!("Unsupported puzzle size: {}x{}"),
        };
        let box_height = side_len / box_width;
        Dynamic {
            data: data,
            nrows: side_len,
            ncols: side_len,
            box_width: box_width,
            box_height: box_height,
        }
    }
}

impl Index<Position> for Dynamic {
    type Output = char;
    fn index<'a>(&'a self, pos: Position) -> &'a Self::Output {
        &self.data[pos.row + pos.col]
    }
}

impl IndexMut<Position> for Dynamic {
    fn index_mut<'a>(&'a mut self, pos: Position) -> &'a mut Self::Output {
        &mut self.data[pos.row + pos.col]
    }
}

#[cfg(test)]
mod tests {
    use super::Dynamic;

    #[test]
    fn from_rows() {
        let rows = vec![vec!['1', '2', '3', '4'],
                        vec!['3', '4', '1', '2'],
                        vec!['2', '1', '4', '3'],
                        vec!['4', '3', '2', '1']
                       ];
        Dynamic::from_rows(&rows);
    }
}
