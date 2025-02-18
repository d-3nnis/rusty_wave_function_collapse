use std::{collections::HashSet, fmt::{self, Debug}, hash::Hash, sync::Arc};

pub type PossibleValue<T> = Arc<Tile<T>>;
pub type PossibleValues<T> = HashSet<PossibleValue<T>>;

pub trait TileType: Eq + Hash + Clone + Debug {}
// impl<T: Eq + Hash + Clone> TileType for T {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Tile<T: TileType> {
    pub id: T,
    pub name: String,
    pub weight: i32,
}

impl<T: TileType> Tile<T> {
    pub fn new(id: T, name: &str, weight: i32) -> Arc<Self> {
        Arc::new(Tile {
            id,
            name: name.to_string(),
            weight,
        })
    }
}

impl<T: TileType> fmt::Debug for Tile<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile {{name: {}}}", self.name)
    }
}
