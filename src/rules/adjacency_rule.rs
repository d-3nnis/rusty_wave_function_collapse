use crate::{
    adjacency_graph::{self, AdjacencyGraph},
    types::{PossibleValues, TileType},
};

use super::Rule;

pub struct AdjacencyRule<T: TileType> {
    adjacency_graph: adjacency_graph::AdjacencyGraph<T>,
}

impl<T: TileType> AdjacencyRule<T> {
    pub fn new(adjacency_graph: AdjacencyGraph<T>) -> Self {
        Self { adjacency_graph }
    }
}

impl<T: TileType> Rule<T> for AdjacencyRule<T> {
    fn propagate_constraints(
        &self,
        grid: &mut crate::grid::Grid<T>,
        x: usize,
        y: usize,
    ) -> Result<Vec<(usize, usize)>, String> {
        let cell = grid
            .get_cell(x, y)
            .ok_or_else(|| format!("Cell at ({}, {}) not found", x, y))?
            .clone();
        let mut affected_cells = Vec::new();
        let valid_coordinates = grid.get_valid_coordinates(x, y);
        // println!("valid_coordinates: {:?}", valid_coordinates);

        for (nx, ny) in valid_coordinates {
            let neighbor_cell = grid.get_cell_mut(nx, ny).unwrap();
            // println!("neighbor_cell: {},{} {:?}", nx, ny, neighbor_cell);
            // if neighbor_cell.is_collapsed() {
                // println!("Skipping collapsed cell");
                // continue;
            // }
            let mut allowed_neighbors = PossibleValues::new();
            for possible_value in cell.possible_values.iter() {
                if let Some(valid_neighbors) =
                    self.adjacency_graph.get_valid_neighbors(possible_value)
                {
                    allowed_neighbors.extend(valid_neighbors.clone());
                }
            }

            if neighbor_cell.constrain(&allowed_neighbors) {
                affected_cells.push((nx, ny));
            }
        }

        Ok(affected_cells)
    }
}
