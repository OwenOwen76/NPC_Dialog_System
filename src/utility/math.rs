use bevy::platform::collections::HashSet;
use rand::Rng;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct WeightedPool {
    pub variants: Vec<(&'static str, usize)>,
}

impl WeightedPool {
    pub fn pick(&self) -> &str {
        let total_weight: usize = self.variants.iter().map(|(_, w)| w).sum();
        let mut rng = rand::thread_rng();
        let mut target = rng.gen_range(0..total_weight);

        for (text, weight) in &self.variants {
            if target < *weight {
                return text;
            }
            target -= weight;
        }
        self.variants[0].0
    }
    pub fn from_set(set: &HashSet<&'static str>) -> Self {
        Self {
            variants: set.iter().map(|&s| (s, 1)).collect(),
        }
    }
}
