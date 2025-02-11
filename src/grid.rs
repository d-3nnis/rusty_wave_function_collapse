use std::collections::{HashMap, HashSet};

use crate::types::{PossibleValue, PossibleValues, TileType};

#[derive(Clone, Debug)]
pub struct Grid<T: TileType> {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<Cell<T>>>,
}

#[derive(Debug, Clone)]
pub struct Cell<T: TileType> {
    pub possible_values: PossibleValues<T>,
}

// impl<T: TileType> PartialEq for Cell<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.possible_values
//             .iter()
//             .map(|tile| &tile.id)
//             .collect::<HashSet<_>>() ==
//         other
//             .possible_values
//             .iter()
//             .map(|tile| &tile.id)
//             .collect::<HashSet<_>>()
//     }
// }

impl<T: TileType> Cell<T> {
    pub fn new(possible_values: PossibleValues<T>) -> Self {
        Self { possible_values }
    }

    pub fn is_collapsed(&self) -> bool {
        self.possible_values.len() == 1
    }

    pub fn get_collapsed_value(&self) -> Option<PossibleValue<T>> {
        // println!("possible_values: {:?}", self.possible_values);
        if self.is_collapsed() {
            // TODO should we return a clone? It's an arc, so it should be ok to clone?
            // Some(self.possible_values.iter().cloned().next())
            return Some(self.possible_values.iter().next().unwrap().clone());
        }
        None
    }

    pub fn constrain(&mut self, allowed: &PossibleValues<T>) {
        self.possible_values.retain(|tile| allowed.contains(tile));
    }

    // pub fn collapse(&mut self, value: char) -> Result<(), String> {
    //     if !self.possible_values.contains_key(&value) {
    //         return Err(format!(
    //             "Value {} is not in possible values: {:?}",
    //             value, self.possible_values
    //         ));
    //     }
    //     self.possible_values = HashMap::from([(value, 1.0)]);
    //     Ok(())
    // }
}

impl<T: TileType> Grid<T> {
    pub fn new(width: usize, height: usize, possible_tiles: PossibleValues<T>) -> Self {
        // TODO share identical possible_values between cells?
        let cells = vec![vec![Cell::new(possible_tiles); width]; height];
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get_cells(&self) -> &Vec<Vec<Cell<T>>> {
        &self.cells
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell<T>> {
        self.cells.get_mut(x).and_then(|row| row.get_mut(y))
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell<T>> {
        self.cells.get(x).and_then(|row| row.get(y))
    }

    pub fn get_valid_coordinates(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut valid_coordinates = vec![];
        if x > 0 {
            valid_coordinates.push((x - 1, y));
        }
        if x < self.width - 1 {
            valid_coordinates.push((x + 1, y));
        }
        if y > 0 {
            valid_coordinates.push((x, y - 1));
        }
        if y < self.height - 1 {
            valid_coordinates.push((x, y + 1));
        }
        valid_coordinates
    }

    pub fn get_adjacent_cells(&self, x: usize, y: usize) -> Vec<&Cell<T>> {
        let mut cells = vec![];
        for (nx, ny) in self.get_valid_coordinates(x, y) {
            if let Some(cell) = self.get_cell(nx, ny) {
                cells.push(cell);
            }
        }

        cells
    }
}
