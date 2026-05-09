use crate::utility::math::WeightedPool;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default, Deref, DerefMut)]
pub struct UserInput {
    input: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyCondition {
    Word(&'static str),
    And(Vec<KeyCondition>),
    Or(Vec<KeyCondition>),
}

impl KeyCondition {
    pub fn is_met(&self, input: &str) -> bool {
        match self {
            KeyCondition::Word(word) => input.contains(word),
            KeyCondition::And(conditions) => conditions.iter().all(|c| c.is_met(input)),
            KeyCondition::Or(conditions) => conditions.iter().any(|c| c.is_met(input)),
        }
    }
}

pub fn expand(template: &str, registry: &HashMap<String, WeightedPool>) -> String {
    let mut result = template.to_string();
    let mut loops = 0;

    while result.contains('[') && loops < 50 {
        if let (Some(start), Some(end)) = (result.find('['), result.find(']')) {
            let tag = &result[start + 1..end];

            if let Some(pool) = registry.get(tag) {
                result.replace_range(start..=end, pool.pick());
            } else {
                result.clone().replace_range(start..=end, tag);
            }
        }
        loops += 1;
    }
    result
}
