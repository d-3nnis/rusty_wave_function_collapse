use crate::{
    adjacency_graph::{self, AdjacencyGraph},
    types::TileType,
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
    ) -> Result<(), String> {
        let collapsed_tile = match grid
            .get_cell(x, y)
            .and_then(|cell| cell.get_collapsed_value())
        {
            Some(tile) => tile,
            None => return Err(format!("Cell at {x},{y} has no collapsed value")),
        };
        for (nx, ny) in grid.get_valid_coordinates(x, y) {
            if let Some(neighbor_cell) = grid.get_cell_mut(nx, ny) {
                if let Some(valid_neighbors) =
                    self.adjacency_graph.get_valid_neighbors(&collapsed_tile)
                {
                    // neighbor_cell.remove_possible_value(&collapsed_value);
                    neighbor_cell.constrain(valid_neighbors);
                } else {
                    panic!("No valid neighbors for tile: {:?}", collapsed_tile)
                }
            }
        }
        Ok(())
    }
}
