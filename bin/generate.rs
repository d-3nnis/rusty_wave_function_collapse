use std::{collections::HashMap, io::{self, Write}};

use colored::Color;
use wfc_too::{
    adjacency_graph::AdjacencyGraph,
    renderer::AsciiRenderer,
    rules::{adjacency_rule::AdjacencyRule, Rule},
    traits::{AsciiRenderable, ColorRenderable},
    types::{PossibleValues, Tile, TileType},
    wfc::WFC,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiTile {
    pub id: char,
    pub color: Color,
}

impl std::hash::Hash for AsciiTile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl TileType for AsciiTile {}

impl AsciiRenderable for AsciiTile {
    fn get_ascii_representation(&self) -> char {
        return self.id;
    }
}

impl ColorRenderable for AsciiTile {
    fn get_color(&self) -> Color {
        return self.color;
    }
}

/// Computes and prints the probability breakdown of each tile type
pub fn analyze_initial_tile_probabilities<T: TileType>(possible_tiles: &PossibleValues<T>) {
    let total_weight: f64 = possible_tiles.iter().map(|tile| tile.weight as f64).sum();

    println!("\n Initial Tile Probability Breakdown:");
    let mut probabilities: HashMap<String, f64> = HashMap::new();

    for tile in possible_tiles {
        let probability = (tile.weight as f64) / total_weight * 100.0;
        probabilities.insert(tile.name.clone(), probability);
    }

    for (tile_name, probability) in probabilities.iter() {
        println!("  {}: {:.2}%", tile_name, probability);
    }

    // Wait for user input before continuing
    print!("\nPress Enter to start the algorithm...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}


fn main() {
    let grass = Tile::new(
        self::AsciiTile {
            id: 'g',
            color: Color::Green,
        },
        "Grass",
        2,
    );
    let beach = Tile::new(
        self::AsciiTile {
            id: '.',
            color: Color::Yellow,
        },
        "Beach",
        1,
    );
    let water = Tile::new(
        self::AsciiTile {
            id: '~',
            color: Color::Blue,
        },
        "Water",
        2,
    );
    let hills = Tile::new(
        self::AsciiTile {
            id: '^',
            color: Color::White,
        },
        "Hills",
        2,
    );
    let tile_types: PossibleValues<AsciiTile> = vec![
        beach.clone(),
        grass.clone(),
        water.clone(),
        hills.clone(),
    ]
    .into_iter()
    .collect();

    let mut adj_graph = AdjacencyGraph::new();
    adj_graph.add_self_adjacencies(vec![&grass, &water, &beach, &hills]);
    adj_graph.add_adjacency(&grass, &beach);
    adj_graph.add_adjacency(&beach, &water);
    let adj_rule = AdjacencyRule::new(adj_graph);
    let renderer = Some(AsciiRenderer);
    let rules: Vec<Box<dyn Rule<AsciiTile>>> = vec![
        Box::new(adj_rule),
    ];
    analyze_initial_tile_probabilities(&tile_types);
    let grid_size = 15;
    let mut wfc = WFC::new(grid_size, grid_size+10, tile_types, rules, renderer);
    wfc.preset_tile(water.clone(), 0, 0);
    wfc.preset_tile(water.clone(), 5, 0);
    wfc.preset_tile(water.clone(), 0, 5);
    wfc.preset_tile(grass.clone(), 0, 10);
    wfc.preset_tile(grass.clone(), 10, 0);
    wfc.preset_tile(grass.clone(), 10, 10);

    match wfc.run() {
        Ok(_) => {
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
