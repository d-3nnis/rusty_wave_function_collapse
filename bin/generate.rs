use colored::Color;
use egui::Ui;
use wfc_too::{
    adjacency_graph::AdjacencyGraph,
    grid::Grid,
    renderer::AsciiRenderer,
    rules::{adjacency_rule::AdjacencyRule, Rule},
    traits::{AsciiRenderable, ColorRenderable},
    types::{PossibleValues, Tile, TileType},
    wfc::WFC,
};

use eframe

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiTile {
    pub id: char,
    pub color: Color,
}

pub struct EguiRenderer<T: TileType> {
    grid: Grid<T>,
}

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

pub struct BeachRule;

impl<T: TileType> Rule<T> for BeachRule {
    fn propagate_constraints(
        &self,
        grid: &mut wfc_too::grid::Grid<T>,
        x: usize,
        y: usize,
    ) -> Result<Vec<(usize, usize)>, String> {
        // let mut cell : &mut Cell<T> = grid.get_cell_mut(x, y).unwrap();
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
        // println!("Could be beach");

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
            let before_count = cell.possible_values.len();
            // cell.possible_values.retain(|tile| tile.name != "Beach");
            cell.constrain_by_name("Beach");
            if cell.possible_values.len() < before_count {
                affected_cells.push((x, y));
            }
        }

        Ok(affected_cells)
    }
}

fn main() {
    let grass = Tile::new(
        self::AsciiTile {
            id: 'g',
            color: Color::Green,
        },
        "Grass",
        4,
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
            color: Color::BrightBlue,
        },
        "Water",
        6,
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
        deep_ocean.clone(),
    ]
    .into_iter()
    .collect();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0,240.0]),
        ..Default::default()
    };
    let mut adj_graph = AdjacencyGraph::new();
    adj_graph.add_self_adjacencies(vec![&grass, &water, &beach, &hills, &deep_ocean]);
    // adj_graph.add_adjacency(&grass, &hills);
    adj_graph.add_adjacency(&grass, &beach);
    adj_graph.add_adjacency(&beach, &water);
    adj_graph.add_adjacency(&deep_ocean, &water);
    let adj_rule = AdjacencyRule::new(adj_graph);
    let beach_rule = BeachRule;
    let renderer = Some(AsciiRenderer);
    let rules: Vec<Box<dyn Rule<AsciiTile>>> = vec![Box::new(adj_rule), Box::new(beach_rule)];
    let grid_size = 5;
    // let grid_width = 50;
    // let grid_height = 30;
    // let mut wfc = WFC::new(grid_width, grid_height, tile_types, rules, renderer);
    let mut wfc = WFC::new(grid_size, grid_size, tile_types, rules, renderer);
    wfc.grid.debug_check_shared_cells();
    wfc.grid.debug_check_shared_possible_values();
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
