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
    fn get_row(&self, pos: Position) -> Group<char> {
        let begin = pos.row*self.ncols;
        let end = (pos.row+1)*self.ncols;
        Group::from(&self.data[begin..end])
    }

    fn get_col(&self, pos: Position) -> Group<char>{
        let mut elements = Vec::with_capacity(self.nrows);
        for row in 0..self.nrows {
            elements.push(self.data[row*self.ncols + pos.col]);
        }
        Group::from(&elements[..])
    }

    fn get_box(&self, pos: Position) -> Group<char>{
        let box_num = self.box_height*(pos.row/self.box_height) + (pos.col/self.box_width);
        let boxes_per_row = self.ncols/self.box_width;
        let top_left = (box_num/boxes_per_row)*self.box_height*self.ncols +
                       (box_num%boxes_per_row)*self.box_width;
        let mut elements = Vec::with_capacity(self.box_width*self.box_height);
        for box_row in 0..self.box_height {
            for box_col in 0..self.box_width {
                elements.push(self.data[top_left + box_row*self.ncols + box_col]);
            }
        }
        Group::from(&elements[..])
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

    fn groups(&self, pos: Position) -> [Group<char>; 3] {
        [self.get_row(pos), self.get_col(pos), self.get_box(pos)]
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
