//! Dynamically-sized sudoku

use std::ops::{Index, IndexMut};
use super::super::position::Position;
use super::super::group::Group;
use super::super::sudoku_trait::Sudoku;

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
            _  => panic!("Unsupported puzzle size: {}x{}", side_len, side_len),
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
        &self.data[pos.row*self.ncols + pos.col]
    }
}

impl IndexMut<Position> for Dynamic {
    fn index_mut<'a>(&'a mut self, pos: Position) -> &'a mut Self::Output {
        &mut self.data[pos.row*self.ncols + pos.col]
    }
}

impl Sudoku for Dynamic {
    fn row_positions(&self, pos: Position) -> Group<Position> {
        (0..pos.col).chain(pos.col+1..self.ncols).map(|c| Position{row: pos.row, col: c}).collect()
    }

    fn col_positions(&self, pos: Position) -> Group<Position>{
        (0..pos.row).chain(pos.row+1..self.nrows).map(|r| Position{row: r, col: pos.col}).collect()
    }

    fn box_positions(&self, pos: Position) -> Group<Position>{
        let top_left = Position {
            row: (pos.row/self.box_height)*self.box_height,
            col: (pos.col/self.box_width)*self.box_width,
        };
        (0..self.box_height)
            .flat_map(|r| (0..self.box_width).map(move |c| (r, c)))
            .map(|(r, c)| Position{ row: top_left.row + r, col: top_left.col + c })
            .filter(|&p| p!=pos).collect()
    }

    fn allowed_characters(&self) -> &'static str {
        match self.nrows {
             4 => "1234",
             6 => "123456",
             9 => "123456789",
            16 => "1234567890ABCDEF",
            25 => "1234567890ABCDEFGHIJKLMNO",
             _ => panic!("Unsupported puzzle size: {}x{}", self.nrows, self.nrows),
        }
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
