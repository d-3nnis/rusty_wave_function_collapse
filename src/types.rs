use std::{collections::HashSet, fmt::Debug, hash::Hash, sync::Arc};

pub type PossibleValue<T> = Arc<Tile<T>>;
pub type PossibleValues<T> = HashSet<PossibleValue<T>>;

pub trait TileType: Eq + Hash + Clone + Debug {}
// impl<T: Eq + Hash + Clone> TileType for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile<T: Eq + Hash + Clone> {
    pub id: T,
    pub name: String,
    pub weight: i32,
}

impl<T: Eq + Hash + Clone> Tile<T> {
    pub fn new(id: T, name: &str, weight: i32) -> Arc<Self> {
        Arc::new(Tile {
            id,
            name: name.to_string(),
            weight,
        })
    }
}
