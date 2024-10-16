
#[cfg(test)]
mod tests {
    use crate::utils::tile::Tile;
    use crate::utils::grid::Grid;
    #[test]
    fn basic_test_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    enum TestTileType {
        Grass,
        Water,
        Beach,
        Hill,
        Mountain,
    }


    #[test]
    fn tile_adj_check() {
        let mut grass = Tile::new(1.0, 'x', "green".to_string(), TestTileType::Grass);
        let mut water = Tile::new(1.0, 'x', "blue".to_string(), TestTileType::Water);
        let mut beach = Tile::new(0.5, 'x', "yellow".to_string(), TestTileType::Beach);
        let mut hill = Tile::new(0.4, 'x', "grey".to_string(), TestTileType::Hill);
        let mut mountain = Tile::new(0.1, 'x', "black".to_string(), TestTileType::Mountain);
        
        grass.set_possible_neighbors(vec![TestTileType::Beach, TestTileType::Grass]);
        water.set_possible_neighbors(vec![TestTileType::Water, TestTileType::Beach, TestTileType::Mountain, TestTileType::Hill]);
        beach.set_possible_neighbors(vec![TestTileType::Beach, TestTileType::Grass, TestTileType::Water]);
        hill.set_possible_neighbors(vec![TestTileType::Hill, TestTileType::Mountain, TestTileType::Grass, TestTileType::Beach]);
        mountain.set_possible_neighbors(vec![TestTileType::Mountain, TestTileType::Hill]);
        assert_eq!(grass.can_be_adjacent_to(&beach), true);
        assert_eq!(grass.can_be_adjacent_to(&mountain), false);
        assert_eq!(water.can_be_adjacent_to(&beach), true);
        assert_eq!(water.can_be_adjacent_to(&mountain), true);
        assert_eq!(beach.can_be_adjacent_to(&grass), true);
        assert_eq!(beach.can_be_adjacent_to(&mountain), false);
        assert_eq!(hill.can_be_adjacent_to(&grass), true);
        assert_eq!(hill.can_be_adjacent_to(&mountain), true);
        assert_eq!(mountain.can_be_adjacent_to(&hill), true);
        assert_eq!(mountain.can_be_adjacent_to(&beach), false);
    }
}
