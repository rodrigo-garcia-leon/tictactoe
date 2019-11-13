use std::fmt;
use crate::cell::*;
use crate::utils::*;

pub struct Board<'a> {
    size: usize,
    cells: Vec<&'a Cell> 
}

impl<'a> Board<'a> {
    fn get_cell_at(& self, row: usize, col: usize) -> &Cell {
        return self.cells[get_cell_index(self.size, row, col)];
    }
    pub fn new(size: usize) -> Board<'a> {
        Board { size, cells: vec![&Cell::Empty; size * size] }
    }
    pub fn is_position_valid(& self, position: (usize, usize)) -> bool {
        if position.0 > self.size || position.1 > self.size {
            return false;
        }
        true
    }
    pub fn is_position_free(& self, position: (usize, usize)) -> bool {
        return self.get_cell_at(position.0, position.1) == &Cell::Empty;
    }
    pub fn update(&mut self, position: (usize, usize), symbol: &'a Cell) {
        self.cells[get_cell_index(self.size, position.0, position.1)] = symbol;
    }
    pub fn finished(&mut self) -> bool {
        // rows
        for row in 1..=self.size {
            let slice: &mut Vec<&Cell> = &mut Vec::new();
            for col in 1..=self.size {
                slice.push(self.get_cell_at(row, col));
            }
            if is_slice_complete(slice) {
                return true;
            }
        }

        //cols
        for col in 1..=self.size {
            let slice: &mut Vec<&Cell> = &mut Vec::new();
            for row in 1..=self.size {
                slice.push(self.get_cell_at(row, col));
            }
            if is_slice_complete(slice) {
                return true;
            }
        }

        // diag1
        let slice: &mut Vec<&Cell> = &mut Vec::new();
        for i in 1..=self.size {
            slice.push(self.get_cell_at(i, i));
        }
        if is_slice_complete(slice) {
            return true;
        }

        // diag2
        let slice: &mut Vec<&Cell> = &mut Vec::new();
        for i in 1..=self.size {
            slice.push(self.get_cell_at(i, self.size - i + 1));
        }
        if is_slice_complete(slice) {
            return true;
        }

        false
    }

    pub fn full(&mut self) -> bool {
        for cell in self.cells.iter() {
            if *cell == &Cell::Empty {
                return false
            }
        }
        true
    }
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for (i, cell) in self.cells.iter().enumerate() {
            s.push_str(&cell.to_string());

            if i % self.size != self.size - 1 {
                s.push_str("|");
            } else if  i != self.size * self.size - 1 {
                s.push_str(&format!("\n{}\n", "-".repeat(self.size * 2 - 1)));
            }
        }

        write!(f, "{}", s)
    }
}