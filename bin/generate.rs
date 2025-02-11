use colored::Color;
use wfc_too::adjacency_graph::AdjacencyGraph;
use wfc_too::grid;
use wfc_too::grid::Grid;
use wfc_too::renderer;
use wfc_too::renderer::render_ascii;
use wfc_too::traits::AsciiRenderable;
use wfc_too::traits::ColorRenderable;
use wfc_too::types;
use wfc_too::types::PossibleValues;
use wfc_too::types::Tile;
use wfc_too::types::TileType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiTile {
    pub id: char,
    pub color: Color,
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
    let tile_types: PossibleValues<AsciiTile> =
        vec![beach.clone(), grass.clone()].into_iter().collect();
    // let tile_types : PossibleValues<AsciiTile> = vec![beach.clone()].into_iter().collect();
    let grid = Grid::new(10, 10, tile_types);

    // let mut graph = AdjacencyGraph::new();
    // graph.
    render_ascii(&grid);
}
