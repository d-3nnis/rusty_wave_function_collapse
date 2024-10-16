mod utils;
mod tests;
use crate::utils::tile::Tile;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum TileType {
    Grass,
    Water,
    Beach,
    Hill,
    Mountain,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <width> <height>", args[0]);
        std::process::exit(1);
    }
    let width = args[1].parse::<usize>().unwrap();
    let height = args[2].parse::<usize>().unwrap();
    let mut grass = Tile::new(9.0, '#', "green".to_string(), TileType::Grass);
    let mut water = Tile::new(2.0, '~', "blue".to_string(), TileType::Water);
    let mut beach = Tile::new(4.0, '.', "yellow".to_string(), TileType::Beach);
    let mut hill = Tile::new(5.0, 'v', "grey".to_string(), TileType::Hill);
    let mut mountain = Tile::new(5.0, 'V', "grey".to_string(), TileType::Mountain);
    
    grass.set_possible_neighbors(vec![TileType::Beach, TileType::Grass, TileType::Hill]);
    water.set_possible_neighbors(vec![TileType::Water, TileType::Beach]);
    beach.set_possible_neighbors(vec![TileType::Beach, TileType::Grass, TileType::Water]);
    hill.set_possible_neighbors(vec![TileType::Hill, TileType::Mountain, TileType::Grass, TileType::Beach]);
    mountain.set_possible_neighbors(vec![TileType::Mountain, TileType::Hill]);

    let mut grid = utils::grid::Grid::new(vec![grass, water, beach, hill, mountain], height, width);
    match grid.collapse_all_cells() {
        Ok(_) => println!("Collapsed all cells"),
        Err(e) => println!("Error: {}", e),
    }
    grid.display_text();
}
