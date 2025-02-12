use colored::Color;

use crate::{grid::Grid, types::TileType};

pub trait Renderer<T: TileType> {
    fn render(&self, grid: &Grid<T>);
}

pub trait AsciiRenderable {
    fn get_ascii_representation(&self) -> char;
}

pub trait ColorRenderable {
    fn get_color(&self) -> Color;
}
