mod grid {
    use crate::utils::tile::Tile;
    use colored::Colorize;
    use petgraph::graph::{Graph, NodeIndex};
    use petgraph::Undirected;
    use std::collections::HashMap;
    use std::collections::{HashSet, VecDeque};
    use weighted_rand::builder::*;

    pub trait Rule<T> {
        fn is_valid(&self, tile: &Tile<T>, neighbors: &[&Tile<T>]) -> bool;
    }

    // Wrapper for managing the tile adjacency graph with generic T
    pub struct TileAdjacencyGraph<T> {
        graph: Graph<T, (), Undirected>, // Graph with nodes of type T
        node_map: HashMap<T, NodeIndex>, // Map from T to the node index in the graph
    }

    impl<T: Eq + std::hash::Hash + Clone> TileAdjacencyGraph<T> {
        // Initialize an empty graph
        pub fn new() -> Self {
            TileAdjacencyGraph {
                graph: Graph::new_undirected(), // Use undirected graph for bidirectional relationships
                node_map: HashMap::new(),
            }
        }

        // Add a node to the graph
        pub fn add_tile_type(&mut self, tile_type: T) {
            let node_idx = self.graph.add_node(tile_type.clone());
            self.node_map.insert(tile_type, node_idx);
        }

        // Add a bidirectional edge between two tile types
        pub fn add_bidirectional_edge(&mut self, tile_a: T, tile_b: T) {
            if let (Some(&idx_a), Some(&idx_b)) =
                (self.node_map.get(&tile_a), self.node_map.get(&tile_b))
            {
                self.graph.add_edge(idx_a, idx_b, ()); // Add an undirected edge
            }
        }

        // Check if two tiles can be neighbors
        fn are_neighbors(&self, tile_a: &T, tile_b: &T) -> bool {
            if let (Some(&idx_a), Some(&idx_b)) =
                (self.node_map.get(tile_a), self.node_map.get(tile_b))
            {
                self.graph.contains_edge(idx_a, idx_b)
            } else {
                false
            }
        }
    }

    impl<T: PartialEq + Eq + std::hash::Hash + Clone> Rule<T> for AdjacencyRule<T> {
        fn is_valid(&self, tile: &Tile<T>, neighbors: &[&Tile<T>]) -> bool {
            if self.can_neighbor_self && neighbors.contains(&&tile) {
                return true;
            }
            for neighbor in neighbors {
                if !self.graph.are_neighbors(&tile.tile_id, &neighbor.tile_id) {
                    return false;
                }
            }
            true
        }
    }

    pub struct AdjacencyRule<T> {
        graph: TileAdjacencyGraph<T>,
        can_neighbor_self: bool,
    }

    impl<T: PartialEq + Eq + std::hash::Hash + Clone> AdjacencyRule<T> {
        pub fn new(graph: TileAdjacencyGraph<T>, can_neighbor_self: Option<bool>) -> Self {
            AdjacencyRule {
                graph,
                can_neighbor_self: can_neighbor_self.unwrap_or(true),
            }
        }
    }

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

        pub fn num_possible_values(&self) -> usize {
            self.possible_values.len()
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

        // pub fn can_be_adjacent_to(&self, other: &T) -> bool {
        //     self.possible_values
        //         .iter()
        //         .any(|tile| tile.can_be_adjacent_to(other))
        // }

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

        pub fn get_collapse_tile(&self) -> Result<&Tile<T>, String> {
            match self.is_collapsed() {
                true => Ok(&self.possible_values[0]),
                false => Err("Cell is not collapsed".to_string()),
            }
        }
    }

    pub struct Grid<T> {
        cells: Vec<Vec<Cell<T>>>,
        adjacency_rule: AdjacencyRule<T>,
        complex_rules: Vec<Box<dyn Rule<T>>>,
    }

    impl<T> Grid<T>
    where
        T: Clone + std::fmt::Debug + Eq + PartialEq + std::hash::Hash,
    {
        pub fn new(
            possible_values: Vec<Tile<T>>,
            height: usize,
            width: usize,
            adjacency_rule: AdjacencyRule<T>,
            complex_rules: Vec<Box<dyn Rule<T>>>,
        ) -> Self {
            let cells = vec![vec![Cell::new(possible_values.clone()); width]; height];
            Grid {
                cells,
                adjacency_rule,
                complex_rules,
            }
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
        }

        pub fn collapse_all_cells(&mut self) -> Result<(), String> {
            loop {
                match self.find_lowest_entropy_cell() {
                    Some((idxs, cell)) => {
                        // println!(
                        //     "Collapsing cell at ({}, {}) with possible values: {:?}",
                        //     idxs.0, idxs.1, cell.possible_values
                        // );
                        cell.collapse()?;
                        //self.propegate_constraints(idxs)?;
                        self.propagate_constraints_with_rules(idxs);
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

        fn get_neighbors(&self, idxs: (usize, usize)) -> Vec<&Cell<T>> {
            let (row_idx, col_idx) = idxs;
            let max_row_idx = self.cells.len();
            let max_col_idx = self.cells[0].len();
            let neighbors = vec![
                (Some(row_idx), Some(col_idx + 1)),
                (Some(row_idx), col_idx.checked_sub(1)),
                (row_idx.checked_sub(1), Some(col_idx)),
                (Some(row_idx + 1), Some(col_idx)),
            ];
            neighbors
                .into_iter()
                .filter_map(|(r_option, c_option)| match (r_option, c_option) {
                    (Some(r), Some(c)) if r < max_row_idx && c < max_col_idx => {
                        Some(&self.cells[r][c])
                    }
                    _ => None,
                })
                .collect::<Vec<&Cell<T>>>()
        }

        fn get_neighbor_idxs(&self, idxs: (usize, usize)) -> Option<Vec<(usize, usize)>> {
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

        fn propagate_constraints_with_rules(&mut self, idxs: (usize, usize)) -> Result<(), String> {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            visited.insert(idxs);
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

            // Add neighbors of the current cell to the queue
            if let Some(neighbors) = self.get_neighbor_idxs(idxs) {
                for neighbor in neighbors {
                    //println!("Adding neighbor: {:?}", neighbor);
                    queue.push_back(neighbor);
                }
            }

            while let Some((row_idx, col_idx)) = queue.pop_front() {
                if visited.contains(&(row_idx, col_idx)) {
                    continue;
                }
                //println!("Popped cell at ({}, {})", row_idx, col_idx);
                visited.insert((row_idx, col_idx));

                // Update constraints using the rules system
                let updated = self.update_constraints_with_rules(
                    (row_idx, col_idx), // Current cell
                );

                // Make this nicer
                if updated? {
                    // If constraints were updated, propagate to neighbors
                    if let Some(neighbors) = self.get_neighbor_idxs((row_idx, col_idx)) {
                        //println!("Visited list: {:?}", visited);
                        for neighbor in neighbors {
                            let (neighbor_row_idx, neighbor_col_idx) = neighbor;
                            if !visited.contains(&neighbor)
                                && !self.cells[neighbor_row_idx][neighbor_col_idx].is_collapsed()
                            {
                                // println!(
                                //     "Adding cell at ({}, {}) to queue",
                                //     neighbor_row_idx, neighbor_col_idx
                                // );
                                // HUH???
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        fn update_constraints_with_rules(
            &mut self,
            idxs: (usize, usize), // Index of the current cell being updated
        ) -> Result<bool, String> {
            let (row_idx, col_idx) = idxs;

            let mut adj_cell = self.cells[row_idx][col_idx].clone();
            if adj_cell.is_collapsed() {
                return Ok(false);
                // TODO perhaps we should handle this somehow?
            }

            let initial_possible_val_len = adj_cell.possible_values.len();

            // Get the current cell's neighbors for applying the rule system
            println!(
                "Current possible values for {}, {}: {:?}",
                row_idx, col_idx, adj_cell.possible_values
            );
            let neighbors = &mut self.get_neighbors(idxs);
            adj_cell.possible_values = adj_cell
                .possible_values
                .iter()
                .filter(|tile| {
                    self.adjacency_rule.is_valid(
                        tile,
                        &neighbors
                            .iter()
                            .map(|cell| &cell.possible_values[0])
                            .collect::<Vec<&Tile<T>>>(),
                    )
                })
                .cloned()
                .collect();
            //println!("Updated possible values: {:?}", adj_cell.possible_values);
            // adj_cell
            //     .possible_values
            //     .retain(|tile| adjacency_rule.is_valid(tile, &neighbors));

            // Apply complex rules (like enforcing specific tile patterns)
            for rule in &self.complex_rules {
                // adj_cell
                //     .possible_values
                //     .retain(|tile| rule.is_valid(tile, &neighbors));
            }

            // Check if the possible values have been reduced (constraints updated)
            let updated = adj_cell.possible_values.len() != initial_possible_val_len;

            if updated {
                println!(
                    "Updated possible values for cell at ({}, {}): {:?}",
                    row_idx, col_idx, adj_cell.possible_values
                );
                self.cells[row_idx][col_idx] = adj_cell;
            }

            // Return whether the constraints were updated
            Ok(updated)
        }

        // fn propegate_constraints(&mut self, idxs: (usize, usize)) -> Result<(), String> {
        //     let mut visited: HashSet<(usize, usize)> = HashSet::new();
        //     visited.insert(idxs);
        //     let mut queue: VecDeque<((usize, usize), (usize, usize))> = VecDeque::new();
        //     if let Some(neighbors) = self.get_neighbor_idxs(idxs) {
        //         for neighbor in neighbors {
        //             println!("Adding neighbor: {:?}", neighbor);
        //             queue.push_back((neighbor, idxs));
        //         }
        //         //println!("Neighbors: {:?}", neighbors);
        //     }
        //     //println!("Propegating constraints for cell at {:?}", queue);
        //
        //     while let Some(((row_idx, col_idx), (source_row_idx, source_col_idx))) = queue.pop_front() {
        //         if visited.contains(&(row_idx, col_idx)) {
        //             continue;
        //         }
        //         println!("Popped cell at ({}, {})", row_idx, col_idx);
        //         visited.insert((row_idx, col_idx));
        //         println!("Queue: {:?}", queue);
        //         let updated =
        //             self.update_constraints((row_idx, col_idx), (source_row_idx, source_col_idx));
        //         if updated {
        //             if let Some(neighbors) = self.get_neighbor_idxs((row_idx, col_idx)) {
        //                 println!("Visited list: {:?}", visited);
        //                 for neighbor in neighbors {
        //                     let (neighbor_row_idx, neighbor_col_idx) = neighbor;
        //                     if !visited.contains(&neighbor)
        //                         && !self.cells[neighbor_row_idx][neighbor_col_idx].is_collapsed()
        //                     {
        //                         println!(
        //                             "Adding cell at ({}, {}) to queue",
        //                             neighbor_row_idx, neighbor_col_idx
        //                         );
        //                         queue.push_back(((row_idx, col_idx), neighbor));
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     Ok(())
        // }
        //
        // fn update_constraints(&mut self, idxs: (usize, usize), source_idxs: (usize, usize)) -> bool {
        //     let (row_idx, col_idx) = idxs;
        //     let (source_row_idx, source_col_idx) = source_idxs;
        //     // should we assume indexes valid?
        //     // TODO Fix all this cloning
        //     let adj_cell = &mut self.cells[row_idx][col_idx].clone();
        //     if adj_cell.is_collapsed() {
        //         return false;
        //     }
        //     let source_cell = &self.cells[source_row_idx][source_col_idx];
        //     let initial_possible_val_len = adj_cell.possible_values.len();
        //     adj_cell.possible_values = adj_cell
        //         .possible_values
        //         .iter()
        //         .filter(|tile| source_cell.can_be_adjacent_to(&tile.tile_id))
        //         .cloned()
        //         .collect();
        //     println!(
        //         "Updated possible values for cell at ({}, {}): {:?}",
        //         row_idx, col_idx, adj_cell.possible_values
        //     );
        //     println!(
        //         "Needs update: {:?}",
        //         adj_cell.possible_values.len() != initial_possible_val_len
        //     );
        //     self.cells[row_idx][col_idx] = adj_cell.clone();
        //     return adj_cell.possible_values.len() != initial_possible_val_len;
        // }
    }

    #[cfg(test)]
    mod tests {
        use crate::utils::grid::grid::*;

        #[test]
        fn test_cell_collapse_with_rule() {
            let width = 2;
            let height = 2;

            let tiles = create_n_sample_tiles(2);
            assert_eq!(tiles.len(), 2);

            let mut tile_graph = TileAdjacencyGraph::new();
            for tile in &tiles {
                tile_graph.add_tile_type(tile.tile_id.clone());
            }

            tile_graph.add_bidirectional_edge(tiles[0].tile_id.clone(), tiles[1].tile_id.clone());

            let rule = AdjacencyRule::new(tile_graph, None);
            let mut grid = Grid::new(tiles, height, width, rule, Vec::new());
        }
    }
}
