mod utils;
mod tests;
use utils::grid;

use crate::utils::tile::Tile;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Debug)]
#[derive(Hash)]
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

    let mut tile_graph = grid::TileAdjacencyGraph::new();
    tile_graph.add_tile_type(TileType::Grass);
    tile_graph.add_tile_type(TileType::Water);
    tile_graph.add_tile_type(TileType::Beach);
    tile_graph.add_tile_type(TileType::Hill);
    tile_graph.add_tile_type(TileType::Mountain);

    tile_graph.add_bidirectional_edge(TileType::Grass, TileType::Beach);
    tile_graph.add_bidirectional_edge(TileType::Grass, TileType::Hill);
    tile_graph.add_bidirectional_edge(TileType::Water, TileType::Beach);
    tile_graph.add_bidirectional_edge(TileType::Hill, TileType::Mountain);
    tile_graph.add_bidirectional_edge(TileType::Hill, TileType::Beach);

    let grass = Tile::new(4.0, '#', "green".to_string(), TileType::Grass);
    let water = Tile::new(2.0, '~', "blue".to_string(), TileType::Water);
    let beach = Tile::new(4.0, '.', "yellow".to_string(), TileType::Beach);
    let hill = Tile::new(1.0, 'v', "grey".to_string(), TileType::Hill);
    let mountain = Tile::new(5.0, 'V', "grey".to_string(), TileType::Mountain);

    let rule = AdjacencyRule::new(tile_graph, None);
    let mut grid = utils::grid::Grid::new(vec![grass, water, beach, hill, mountain], height, width, rule, Vec::new());
    match grid.collapse_all_cells() {
        Ok(_) => println!("Collapsed all cells"),
        Err(e) => println!("Error: {}", e),
    }
    grid.display_text();
}
