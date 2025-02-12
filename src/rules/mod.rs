use crate::{
    grid::Grid,
    types::TileType,
};
pub mod adjacency_rule;

pub trait Rule<T: TileType> {
    fn propagate_constraints(
        &self,
        grid: &mut Grid<T>,
        x: usize,
        y: usize,
    ) -> Result<Vec<(usize, usize)>, String>;
}
