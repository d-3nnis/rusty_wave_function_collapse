use colored::Colorize;
use crate::{
    grid::Grid,
    traits::{AsciiRenderable, ColorRenderable},
    types::TileType,
};

pub fn render_ascii<T: TileType + AsciiRenderable + ColorRenderable>(grid: &Grid<T>) {
    for row in grid.get_cells() {
        for cell in row {
            // println!("test_cell: {:?}", test_cell);
            if let Some(tile) = cell.get_collapsed_value() {
                let ascii = tile.id.get_ascii_representation();
                let color = tile.id.get_color();
                print!("{}", ascii.to_string().color(color))
            } else {
                print!("#")
            }
        }
        println!();
    }
}
