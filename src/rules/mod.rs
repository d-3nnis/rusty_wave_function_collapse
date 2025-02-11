use crate::{
    grid::Grid,
    types::{PossibleValue, PossibleValues, TileType},
};
pub mod adjacency_rule;

pub trait Rule<T: TileType> {
    // TODO add return values? Errors?
    // fn filter_possible_values(
    //     &self,
    //     grid: &Grid<T>,
    //     x: usize,
    //     y: usize,
    //     possible_values: &mut PossibleValues<T>,
    // );

    fn propagate_constraints(
        &self,
        grid: &mut Grid<T>,
        x: usize,
        y: usize,
    ) -> Result<(), String>;
}
