use std::{collections::HashMap, io::{self, Write}};

use colored::Color;
// use egui::Ui;
use wfc_too::{
    adjacency_graph::AdjacencyGraph,
    grid::Grid,
    renderer::AsciiRenderer,
    rules::{adjacency_rule::AdjacencyRule, Rule},
    traits::{AsciiRenderable, ColorRenderable},
    types::{PossibleValues, Tile, TileType},
    wfc::WFC,
};

// use eframe

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiTile {
    pub id: char,
    pub color: Color,
}

// pub struct EguiRenderer<T: TileType> {
//     grid: Grid<T>,
// }

// hash implementation for AsciiTile
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

pub struct BeachCleanupRule;

impl<T: TileType> Rule<T> for BeachCleanupRule {
    fn propagate_constraints(
        &self,
        grid: &mut wfc_too::grid::Grid<T>,
        x: usize,
        y: usize,
    ) -> Result<Vec<(usize, usize)>, String> {
        let mut affected_cells = Vec::new();

        // Only check for beach tiles
        if !grid
            .get_cell(x, y)
            .unwrap()
            .possible_values
            .iter()
            .any(|tile| tile.name == "Beach")
        {
            return Ok(affected_cells);
        }

        let mut has_water = false;
        let mut has_grass = false;

        for (nx, ny) in grid.get_valid_coordinates(x, y) {
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor
                    .possible_values
                    .iter()
                    .any(|tile| tile.name == "Water")
                {
                    has_water = true;
                }
                if neighbor
                    .possible_values
                    .iter()
                    .any(|tile| tile.name == "Grass")
                {
                    has_grass = true;
                }
            }
        }

        // If a beach tile doesn't have both grass and water nearby, remove it
        if !(has_water && has_grass) {
            println!("Removing invalid beach at ({}, {})", x, y);
            let cell = grid.get_cell_mut(x, y).unwrap();
            if !cell.is_collapsed() {
                if cell.constrain_by_name("Beach") {
                    affected_cells.push((x, y));
                }
            }
        }

        Ok(affected_cells)
    }
}

pub struct BeachEnforcementRule;


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
    let deep_ocean = Tile::new(
        self::AsciiTile {
            id: 'o',
            color: Color::Blue,
        },
        "Deep Ocean",
        1,
    );
    let tile_types: PossibleValues<AsciiTile> = vec![
        beach.clone(),
        grass.clone(),
        water.clone(),
        hills.clone(),
        // deep_ocean.clone(),
    ]
    .into_iter()
    .collect();

    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([320.0,240.0]),
    //     ..Default::default()
    // };
    let mut adj_graph = AdjacencyGraph::new();
    adj_graph.add_self_adjacencies(vec![&grass, &water, &beach, &hills /* &deep_ocean */]);
    // adj_graph.add_adjacency(&grass, &hills);
    adj_graph.add_adjacency(&grass, &beach);
    adj_graph.add_adjacency(&beach, &water);
    adj_graph.add_adjacency(&deep_ocean, &water);
    let adj_rule = AdjacencyRule::new(adj_graph);
    // let beach_rule = BeachCleanupRule;
    let beach_enforcement_rule = BeachEnforcementRule;
    let renderer = Some(AsciiRenderer);
    let rules: Vec<Box<dyn Rule<AsciiTile>>> = vec![
        Box::new(adj_rule),
        // Box::new(beach_rule),
        // Box::new(beach_enforcement_rule),
    ];
    analyze_initial_tile_probabilities(&tile_types);
    let grid_size = 5;
    // let grid_width = 50;
    // let grid_height = 30;
    // let mut wfc = WFC::new(grid_width, grid_height, tile_types, rules, renderer);
    let mut wfc = WFC::new(grid_size, grid_size, tile_types, rules, renderer);
    wfc.preset_tile(water.clone(), 0, 0);
    wfc.preset_tile(grass.clone(), 4, 3);
    wfc.preset_tile(grass.clone(), 3, 4);


    // based on all the tile weights, print the percentage chance of collapse for each type please!
    // Then wait for input.

    match wfc.run() {
        Ok(_) => {
            // let string = "Nice".color("green");
            // println!("{}", string);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
