use crate::utils::tile::Tile;
use colored::ColoredString;
use colored::Colorize;
use rand::seq::SliceRandom;
use std::collections::{HashSet, VecDeque};
use weighted_rand::builder::*;

#[derive(Clone, Debug)]
pub struct Cell<T> {
    possible_values: Vec<Tile<T>>,
}

impl<T> Cell<T>
where
    T: Clone + std::fmt::Debug + PartialEq,
{
    pub fn new(possible_values: Vec<Tile<T>>) -> Self {
        Cell { possible_values }
    }

    pub fn collapse(&mut self) -> Result<(), String> {
        if self.possible_values.len() == 0 {
            return Err("No possible values to collapse".to_string());
        }
        let builder = WalkerTableBuilder::new(
            &self
                .possible_values
                .iter()
                .map(|tile| tile.weight)
                .collect::<Vec<f32>>(),
        );
        let wa_table = builder.build();
        let chosen_idx = wa_table.next();
        let chosen = self.possible_values[chosen_idx].clone();
        self.possible_values = vec![chosen];
        println!("Collapsed cell to {:?}", self.possible_values);
        Ok(())
    }

    pub fn can_be_adjacent_to(&self, other: &T) -> bool {
        self.possible_values
            .iter()
            .any(|tile| tile.can_be_adjacent_to(other))
    }

    pub fn is_collapsed(&self) -> bool {
        self.possible_values.len() == 1
    }

    pub fn get_symbol(&self) -> Option<colored::ColoredString> {
        Some(
            self.possible_values
                .get(0)?
                .symbol
                .to_string()
                .color(self.possible_values[0].color.clone()),
        )
    }
}

pub struct Grid<T> {
    cells: Vec<Vec<Cell<T>>>,
}

impl<T> Grid<T>
where
    T: Clone + std::fmt::Debug + PartialEq,
{
    pub fn new(possible_values: Vec<Tile<T>>, height: usize, width: usize) -> Self {
        let cells = vec![vec![Cell::new(possible_values.clone()); width]; height];
        Grid { cells }
    }

    // TODO make grid_valid() function?

    pub fn display_text(&self) {
        if self.cells.len() == 0 {
            println!("Grid is empty");
            return;
        }
        for row in self.cells.iter() {
            for cell in row.iter() {
                let symbol = cell.get_symbol().unwrap_or("█".to_string().color("white"));
                print!("{}", symbol);
            }
            println!();
        }
    }

    pub fn find_lowest_entropy_cell(&mut self) -> Option<((usize, usize), &mut Cell<T>)> {
        self.cells
            .iter_mut()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(col_idx, cell)| ((row_idx, col_idx), cell))
            })
            .filter(|(_, cell)| !cell.is_collapsed())
            .min_by(|(_, a), (_, b)| a.possible_values.len().cmp(&b.possible_values.len()))
        // Find lowest entropy
        // .flatten()
        // .filter(|cell| cell.possible_values.len() > 1)
        // .min_by(|a, b| a.possible_values.len().cmp(&b.possible_values.len()))
    }

    pub fn collapse_all_cells(&mut self) -> Result<(), String> {
        loop {
            match self.find_lowest_entropy_cell() {
                Some((idxs, cell)) => {
                    println!(
                        "Collapsing cell at ({}, {}) with possible values: {:?}",
                        idxs.0, idxs.1, cell.possible_values
                    );
                    cell.collapse()?;
                    self.propegate_constraints(idxs)?;
                }
                None => {
                    println!("All cells have been collapsed");
                    break;
                    //return Ok(());
                }
            }
        }
        return Ok(());
    }

    fn get_neighbors(&self, idxs: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let (row_idx, col_idx) = idxs;
        let max_row_idx = self.cells.len();
        let max_col_idx = self.cells[0].len();
        let neighbors = vec![
            (Some(row_idx), Some(col_idx + 1)),
            (Some(row_idx), col_idx.checked_sub(1)),
            (row_idx.checked_sub(1), Some(col_idx)),
            (Some(row_idx + 1), Some(col_idx)),
        ];
        Some(
            neighbors
                .into_iter()
                .filter_map(|(r_option, c_option)| match (r_option, c_option) {
                    (Some(r), Some(c)) if r < max_row_idx && c < max_col_idx => Some((r, c)),
                    _ => None,
                })
                .collect(),
        )
    }

    fn propegate_constraints(&mut self, idxs: (usize, usize)) -> Result<(), String> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(idxs);
        let mut queue: VecDeque<((usize, usize), (usize, usize))> = VecDeque::new();
        if let Some(neighbors) = self.get_neighbors(idxs) {
            for neighbor in neighbors {
                println!("Adding neighbor: {:?}", neighbor);
                queue.push_back((neighbor, idxs));
            }
            //println!("Neighbors: {:?}", neighbors);
        }
        //println!("Propegating constraints for cell at {:?}", queue);

        while let Some(((row_idx, col_idx), (source_row_idx, source_col_idx))) = queue.pop_front() {
            if visited.contains(&(row_idx, col_idx)) {
                continue;
            }
            println!("Popped cell at ({}, {})", row_idx, col_idx);
            visited.insert((row_idx, col_idx));
            println!("Queue: {:?}", queue);
            let updated =
                self.update_constraints((row_idx, col_idx), (source_row_idx, source_col_idx));
            if updated {
                if let Some(neighbors) = self.get_neighbors((row_idx, col_idx)) {
                    println!("Visited list: {:?}", visited);
                    for neighbor in neighbors {
                        let (neighbor_row_idx, neighbor_col_idx) = neighbor;
                        if !visited.contains(&neighbor)
                            && !self.cells[neighbor_row_idx][neighbor_col_idx].is_collapsed()
                        {
                            println!(
                                "Adding cell at ({}, {}) to queue",
                                neighbor_row_idx, neighbor_col_idx
                            );
                            queue.push_back(((row_idx, col_idx), neighbor));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn update_constraints(&mut self, idxs: (usize, usize), source_idxs: (usize, usize)) -> bool {
        let (row_idx, col_idx) = idxs;
        let (source_row_idx, source_col_idx) = source_idxs;
        // should we assume indexes valid?
        // TODO Fix all this cloning
        let adj_cell = &mut self.cells[row_idx][col_idx].clone();
        if adj_cell.is_collapsed() {
            return false;
        }
        let source_cell = &self.cells[source_row_idx][source_col_idx];
        let initial_possible_val_len = adj_cell.possible_values.len();
        adj_cell.possible_values = adj_cell
            .possible_values
            .iter()
            .filter(|tile| source_cell.can_be_adjacent_to(&tile.tile_id))
            .cloned()
            .collect();
        println!(
            "Updated possible values for cell at ({}, {}): {:?}",
            row_idx, col_idx, adj_cell.possible_values
        );
        println!(
            "Needs update: {:?}",
            adj_cell.possible_values.len() != initial_possible_val_len
        );
        self.cells[row_idx][col_idx] = adj_cell.clone();
        return adj_cell.possible_values.len() != initial_possible_val_len;
    }
}

