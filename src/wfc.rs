use crate::grid::Grid;
use rand::seq::SliceRandom;

pub struct WFC {
    // grid: Grid,
}

impl WFC {
    // pub fn new(width: usize, height: usize, possible_values: PossibleValues) -> Self {
    //     Self {
    //         grid: Grid::new(width, height, possible_values),
    //     }
    // }
    //
    // fn find_lowest_entropy_cell(&self) -> (usize, usize) {
    //     let mut lowest_entropy = usize::MAX;
    //     let mut lowest_entropy_cell = (0, 0);
    //     for (x, row) in self.grid.cells.iter().enumerate() {
    //         for (y, cell) in row.iter().enumerate() {
    //             if cell.is_collapsed() {
    //                 continue;
    //             }
    //             let entropy = cell.possible_values.len();
    //             if entropy < lowest_entropy {
    //                 lowest_entropy = entropy;
    //                 lowest_entropy_cell = (x, y);
    //             }
    //         }
    //     }
    //
    //     lowest_entropy_cell
    // }
    //
    // fn collapse_lowest_entropy_cell(&mut self) {
    //     let (x, y) = self.find_lowest_entropy_cell();
    //     let cell = self.grid.get_cell(x, y);
    //     // let cell = &mut self.grid.cells[x][y];
    //     // cell.collapse(value).unwrap();
    // }
}
