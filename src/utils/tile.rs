// trait TileType {
//     fn get_id(&self) -> String;
//     fn get_valid_neighbors(&self) -> Vec<String>;
// }

#[derive(Clone, Debug, PartialEq)]
pub struct Tile<T> {
    pub weight: f32,
    pub tile_id: T,
    //pub possible_neighbors: Vec<T>,
    //pub possible_neighbors : Vec<&Tile>,

    // these should be abstracted away
    pub symbol: char,
    pub color: String,
}

// impl<T> PartialEq for Tile<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.tile_id == other.tile_id
//     }
// }

impl<T> Tile<T>
where
    T: Clone + std::fmt::Debug + PartialEq,
{
    pub fn new(weight: f32, symbol: char, color: String, tile_id: T) -> Tile<T> {
        Tile {
            weight,
            symbol,
            color,
            tile_id,
            //possible_neighbors: Vec::new(),
        }
    }

    // pub fn set_possible_neighbors(&mut self, neighbors: Vec<T>) {
    //     self.possible_neighbors = neighbors;
    // }
    //
    // pub fn can_be_adjacent_to(&self, other: &T) -> bool {
    //     self.possible_neighbors.contains(other)
    // }
}
