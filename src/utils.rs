use crate::cell::*;

pub fn get_cell_index(size: usize, row: usize, col: usize) -> usize {
    return (col - 1) + size * (row - 1);
}

pub fn is_slice_full(slice: &Vec<&Cell>) -> bool {
    let mut is_full = true;

    for i in 0..slice.len() - 1 {
        if slice[i] == &Cell::Empty || slice[i + 1] == &Cell::Empty {
            return false;
        }

        is_full = is_full && slice[i] == slice[i + 1];
    }

    is_full
}