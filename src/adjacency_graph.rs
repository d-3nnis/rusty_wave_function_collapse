use crate::types::{PossibleValue, PossibleValues, TileType};
use std::
    collections::{HashMap, HashSet}
;

#[derive(Debug, Clone)]
pub struct AdjacencyGraph<T: TileType> {
    graph: HashMap<PossibleValue<T>, HashSet<PossibleValue<T>>>,
}

impl<T: TileType> AdjacencyGraph<T> {
    // add a vec version that takes all the types? idk
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    pub fn add_self_adjacencies(&mut self, a: Vec<&PossibleValue<T>>) {
        for tile in a {
            self.add_self_adjacency(&tile);
        }
    }

    pub fn add_self_adjacency(&mut self, a: &PossibleValue<T>) {
        self.graph
            .entry(a.clone())
            .or_insert_with(HashSet::new)
            .insert(a.clone());
    }

    pub fn add_adjacency(&mut self, a: &PossibleValue<T>, b: &PossibleValue<T>) {
        self.graph
            .entry(a.clone())
            .or_insert_with(HashSet::new)
            .insert(b.clone());
        self.graph
            .entry(b.clone())
            .or_insert_with(HashSet::new)
            .insert(a.clone());
    }

    pub fn is_valid_neighbor(&self, a: &PossibleValue<T>, b: &PossibleValue<T>) -> bool {
        self.graph
            .get(a)
            .map_or(false, |neighbors| neighbors.contains(b))
    }

    pub fn get_valid_neighbors(&self, tile: &PossibleValue<T>) -> Option<&PossibleValues<T>> {
        // self.graph.get(tile).cloned().unwrap_or_default()
        self.graph.get(tile)
    }
}
