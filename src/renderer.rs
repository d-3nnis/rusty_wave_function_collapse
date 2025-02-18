use std::{
    io::{stdout, Write},
    thread::sleep,
};

use crate::{
    grid::Grid,
    traits::{AsciiRenderable, ColorRenderable, Renderer},
    types::TileType,
};
use colored::Colorize;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};

pub struct AsciiRenderer;

impl<T: TileType + AsciiRenderable + ColorRenderable> Renderer<T> for AsciiRenderer {
    fn render(&self, grid: &Grid<T>) {
        // render_ascii(grid);
        // simple_render(grid);
        debug_render(grid);
    }
}

pub fn render_ascii<T: TileType + AsciiRenderable + ColorRenderable>(grid: &Grid<T>) {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    for row in grid.get_cells() {
        for cell in row {
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
    stdout.flush().unwrap();
    let sleep_time = std::time::Duration::from_millis(15);
    sleep(sleep_time);
}

pub fn simple_render<T: TileType + AsciiRenderable + ColorRenderable>(grid: &Grid<T>) {
    for row in grid.get_cells() {
        for cell in row {
            // println!("cell: {:?}", cell);
            if let Some(tile) = cell.get_collapsed_value() {
                let ascii = tile.id.get_ascii_representation();
                let color = tile.id.get_color();
                print!("{}", ascii.to_string().color(color))
            } else {
                print!("{}", "#".color("white"))
            }
        }
        println!();
    }
}

pub fn debug_render<T: TileType + AsciiRenderable + ColorRenderable>(grid: &Grid<T>) {
    // Determine the maximum number of possibilities in any cell
    let max_options = grid
        .get_cells()
        .iter()
        .flatten()
        .map(|cell| cell.possible_values.len())
        .max()
        .unwrap_or(1); // Ensure at least 1 for collapsed cells

    for row in grid.get_cells() {
        for cell in row {
            if let Some(tile) = cell.get_collapsed_value() {
                let ascii = tile.id.get_ascii_representation();
                let color = tile.id.get_color();
                print!(
                    "[{:^width$}]",
                    ascii.to_string().color(color),
                    width = max_options
                );
            } else {
                let mut possibilities: Vec<String> = cell
                    .possible_values
                    .iter()
                    .map(|tile| {
                        let ascii = tile.id.get_ascii_representation();
                        let color = tile.id.get_color();
                        ascii.to_string().color(color).to_string()
                    })
                    .collect();

                // Ensure the printed length is consistent
                while possibilities.len() < max_options {
                    possibilities.push(" ".to_string()); // Add padding
                }

                print!("[{}]", possibilities.join(""));
            }
            print!(" "); // Space between cells
        }
        println!();
    }
    println!();
}
