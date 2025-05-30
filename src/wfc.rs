use std::usize;

use crate::{
    grid::Grid,
    rules::Rule,
    traits::Renderer,
    types::{PossibleValue, PossibleValues, TileType},
};

pub struct WFC<T: TileType, R: Renderer<T>> {
    pub grid: Grid<T>,
    rules: Vec<Box<dyn Rule<T>>>,
    renderer: Option<R>,
}

// TODO move me to entropy file?
fn calculate_shannon_entropy<T: TileType>(possible_values: &PossibleValues<T>) -> f64 {
    let total_weight: f64 = possible_values.iter().map(|tile| tile.weight as f64).sum();
    if total_weight == 0.0 {
        return 0.0; // Prevent log(0) errors
    }

    possible_values
        .iter()
        .map(|tile| {
            let p = (tile.weight as f64) / total_weight;
            -p * p.log2()
        })
        .sum()
}

impl<T: TileType, R: Renderer<T>> WFC<T, R> {
    pub fn new(
        width: usize,
        height: usize,
        possible_values: PossibleValues<T>,
        rules: Vec<Box<dyn Rule<T>>>,
        renderer: Option<R>,
    ) -> Self {
        Self {
            grid: Grid::new(width, height, possible_values),
            rules,
            renderer,
        }
    }

    fn find_lowest_shannon_entropy_cell(&self) -> Option<(usize, usize)> {
        let mut lowest_entropy = f64::INFINITY; // Start with a large value
        let mut best_candidate = None;

        for (x, row) in self.grid.get_cells().iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if cell.is_collapsed() {
                    continue;
                }

                let entropy = calculate_shannon_entropy(&cell.possible_values);
                if entropy < lowest_entropy {
                    lowest_entropy = entropy;
                    best_candidate = Some((x, y));
                }
            }
        }

        best_candidate
    }

    pub fn run(&mut self) -> Result<(), String> {
        while let Some((x, y)) = self.find_lowest_shannon_entropy_cell() {
            let _cell = self.grid.collapse_cell(x, y)?;

            assert!(self.grid.get_cell(x, y).unwrap().is_collapsed());
            self.propagate_all_constraints(vec![(x, y)]);

            if let Some(renderer) = &self.renderer {
                renderer.render(&self.grid);
            }
        }
        Ok(())
    }

    pub fn propagate_all_constraints(&mut self, start_cells: Vec<(usize, usize)>) {
        let mut queue = start_cells;
        while let Some((cx, cy)) = queue.pop() {
            for rule in self.rules.iter() {
                match rule.propagate_constraints(&mut self.grid, cx, cy) {
                    Ok(mut affected_cells) => queue.append(&mut affected_cells),
                    Err(e) => println!("Error propagating constraints: {}", e),
                }
            }
        }
    }

    pub fn preset_tile(&mut self, value: PossibleValue<T>, x: usize, y: usize) {
        if let Some(cell) = self.grid.get_cell_mut(x, y) {
            cell.possible_values = std::iter::once(value).collect();
            println!("Presetting cell ({}, {}) to {:?}", x, y, cell.possible_values);

            self.propagate_all_constraints(vec![(x, y)]);
        }
    }

    pub fn debug_render(&self) {
        if let Some(renderer) = &self.renderer {
            renderer.render(&self.grid);
        } else {
            println!("No renderer available for debug rendering.");
        }
    }
}
