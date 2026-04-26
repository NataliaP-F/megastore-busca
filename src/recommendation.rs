use std::collections::{HashMap, HashSet};

pub struct RecommendationGraph {
    pub edges: HashMap<u32, HashSet<u32>>,
}

impl RecommendationGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_relation(&mut self, a: u32, b: u32) {
        self.edges.entry(a).or_insert(HashSet::new()).insert(b);
        self.edges.entry(b).or_insert(HashSet::new()).insert(a);
    }

    pub fn recommend(&self, id: u32) -> Vec<u32> {
        match self.edges.get(&id) {
            Some(v) => v.iter().cloned().collect(),
            None => vec![],
        }
    }
}