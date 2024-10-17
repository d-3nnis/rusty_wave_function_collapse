#[cfg(test)]
mod tests {
    use crate::utils::grid::{AdjacencyRule, Cell, Grid, TileAdjacencyGraph};
    use crate::utils::tile::Tile;
    #[test]
    fn basic_test_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn tile_test() {
        let tile = Tile::new(1.0, 'a', "red".to_string(), "test".to_string());
        assert_eq!(tile.weight, 1.0);
        assert_eq!(tile.symbol, 'a');
        assert_eq!(tile.color, "red".to_string());
        assert_eq!(tile.tile_id, "test".to_string());
    }

    fn create_four_sample_tiles() -> Vec<Tile<String>> {
        vec![
            Tile::new(1.0, 'g', "green".to_string(), "grass".to_string()),
            Tile::new(1.0, 'w', "blue".to_string(), "water".to_string()),
            Tile::new(1.0, 'h', "grey".to_string(), "hill".to_string()),
            Tile::new(1.0, 'b', "yellow".to_string(), "beach".to_string()),
        ]
    }

    fn create_n_sample_tiles(num_tiles: i32) -> Vec<Tile<String>> {
        let mut tiles = Vec::new();
        for i in 0..num_tiles {
            tiles.push(Tile::new(1.0, 'a', "red".to_string(), i.to_string()));
        }
        tiles
    }

    #[test]
    fn cell_creation() {
        let tiles = create_four_sample_tiles();
        let tiles_len = tiles.len();
        let cell = Cell::new(tiles);
        assert_eq!(cell.num_possible_values(), tiles_len);
    }

    #[test]
    fn test_single_cell_collapse() {
        let tiles = create_four_sample_tiles();
        let tiles_clone = tiles.clone();
        let mut cell = Cell::new(tiles);
        match cell.collapse() {
            Ok(_) => {
                match cell.get_collapse_tile() {
                    Ok(tile) => {
                        //assert_eq!(tile.to_owned(), tiles_clone[0]);
                        assert!(tiles_clone.contains(&tile));
                    }
                    Err(e) => {
                        panic!("Should be able to get collapsed tile: {e}");
                    }
                }
                assert!(cell.is_collapsed());
                assert_eq!(cell.num_possible_values(), 1);
            }
            Err(e) => {
                panic!("This cell should have collapsed: {e}");
            }
        }
    }

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
