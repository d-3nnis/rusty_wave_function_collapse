use std::collections::HashSet;

use rand::{rng, seq::IndexedRandom};

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

    pub fn constrain_by_name(&mut self, allowed: &str) -> bool {
        let initial_len = self.possible_values.len();
        self.possible_values.retain(|tile| &tile.name != allowed);
        assert!(self.possible_values.len() != 0);
        return initial_len != self.possible_values.len();
    }

    pub fn constrain_by_names(&mut self, allowed: Vec<&str>) -> bool {
        let initial_len = self.possible_values.len();
        self.possible_values
            .retain(|tile| !allowed.contains(&&tile.name[..]));
        assert!(self.possible_values.len() != 0);
        return initial_len != self.possible_values.len();
    }

    pub fn constrain(&mut self, allowed: &PossibleValues<T>) -> bool {
        let initial_len = self.possible_values.len();
        self.possible_values.retain(|tile| allowed.contains(tile));
        assert!(self.possible_values.len() != 0);
        return initial_len != self.possible_values.len();
    }

    pub fn collapse(&mut self) -> Result<PossibleValue<T>, String> {
        if self.is_collapsed() {
            return Err("Cell is already collapsed".to_string());
        }
        let mut rng = rng();
        // println!(
        //     "Collapsing cell with possible_values: {:?}",
        //     self.possible_values
        // );
        match self
            .possible_values
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .choose_weighted(&mut rng, |tile| tile.weight)
            .to_owned()
        {
            Ok(chosen_tile) => {
                // println!("Chosen tile: {:?}", chosen_tile);
                self.possible_values = HashSet::from([chosen_tile.clone()]);
                // println!("Collapsing to {:?}", chosen_tile);
                return Ok(chosen_tile.clone());
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }
}

impl<T: TileType> Grid<T> {
    pub fn new(width: usize, height: usize, possible_tiles: PossibleValues<T>) -> Self {
        // TODO share identical possible_values between cells?
        let cells = vec![vec![Cell::new(possible_tiles); height]; width];
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

    pub fn collapse_cell(&mut self, x: usize, y: usize) -> Result<PossibleValue<T>, String> {
        match self.get_cell_mut(x, y) {
            Some(cell) => {
                if cell.is_collapsed() {
                    return Err(format!("Cell at ({}, {}) is already collapsed", x, y));
                }

                match cell.collapse() {
                    Ok(possible_value) => {
                        return Ok(possible_value);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            None => {
                return Err(format!("Cell at ({}, {}) does not exist", x, y));
            }
        }
    }

    pub fn debug_check_shared_cells(&self) {
        if self.cells.is_empty() || self.cells[0].is_empty() {
            println!("Grid is empty.");
            return;
        }

        let first_cell = &self.cells[0][0];

        let mut all_same = true;
        for (_, row) in self.cells.iter().enumerate() {
            for (_, cell) in row.iter().enumerate() {
                if !std::ptr::eq(first_cell, cell) {
                    all_same = false;
                }
            }
        }

        if all_same {
            println!("Warning: All grid cells share the same reference!");
        } else {
            println!("Grid cells are uniquely allocated.");
        }
    }

    pub fn debug_check_shared_possible_values(&self) {
        if self.cells.is_empty() || self.cells[0].is_empty() {
            println!("Grid is empty.");
            return;
        }

        let first_possible_values = &self.cells[0][0].possible_values;
        let mut all_same = true;

        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if std::ptr::eq(first_possible_values, &cell.possible_values) {
                    println!(
                        "Cell ({}, {}) shares the same possible_values reference!",
                        x, y
                    );
                } else {
                    all_same = false;
                }
            }
        }

        if all_same {
            println!("Warning: All cells share the same `possible_values` set!");
        } else {
            println!("Each cell has a unique `possible_values` set.");
        }
    }
}
