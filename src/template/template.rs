use bevy::platform::collections::HashMap;
use bevy::platform::collections::HashSet;
use bevy::prelude::*;

use crate::template::{mood::*, personality::*};
use crate::utility::{input::*, math::*};

pub trait Talkable {
    fn respond(&mut self, current_mood: &Mood, input: &str) -> String;
}

#[derive(Deref, Component, Debug)]
pub struct Npc {
    #[deref]
    pub name: String,
    pub mood_logic: HashMap<Mood, (WeightedPool, WeightedPool)>,
    pub known_ideas: Vec<(String, KeyCondition, WeightedPool)>,
    pub key_ideas: Vec<(KeyCondition, WeightedPool)>,
    pub memory: Memory,
}

impl Npc {
    pub fn new(
        name: String,
        mood_logic: HashMap<Mood, (WeightedPool, WeightedPool)>,
        known_ideas: Vec<(String, KeyCondition, WeightedPool)>,
        key_ideas: Vec<(KeyCondition, WeightedPool)>,
        memory: Memory,
    ) -> Self {
        Self {
            name,
            mood_logic,
            known_ideas,
            key_ideas,
            memory,
        }
    }
    pub fn update_memory(&mut self, input: &str) {
        let input_low = input.to_lowercase();

        for (cond, _) in &self.key_ideas {
            if cond.is_met(&input_low) {
                match cond {
                    KeyCondition::Word(w) => {
                        self.memory.ideas_said.insert(w.to_string());
                    }
                    KeyCondition::And(list) => {
                        if let Some(KeyCondition::Word(first_word)) = list.first() {
                            self.memory.ideas_said.insert(first_word.to_string());
                        }
                    }
                    KeyCondition::Or(list) => {
                        for sub_cond in list {
                            if let KeyCondition::Word(w) = sub_cond {
                                if input_low.contains(w) {
                                    self.memory.ideas_said.insert(w.to_string());
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn print_debug(&self) {
        println!("\n------");
        println!("\n{:?}", self.known_ideas);
        println!("\n{:?}", self.key_ideas);
        println!("\n{:?}\n", self.memory.ideas_said);
        println!("------")
    }
    pub fn expand_string(
        &self,
        template: &str,
        registry: &HashMap<String, WeightedPool>,
    ) -> String {
        let mut result = template.to_string();

        while let Some(start) = result.find('[') {
            if let Some(end) = result[start..].find(']') {
                let full_end = start + end;
                let tag = &result[start + 1..full_end];

                if let Some(pool) = registry.get(tag) {
                    let replacement = pool.pick();
                    result.replace_range(start..=full_end, replacement);
                } else {
                    result.clone().replace_range(start..=full_end, tag);
                }
            } else {
                break;
            }
        }
        result
    }
}

impl Talkable for Npc {
    fn respond(&mut self, current_mood: &Mood, input: &str) -> String {
        let input_low = input.to_lowercase();

        let (openers, dismissals) =
            self.mood_logic
                .get(current_mood)
                .cloned()
                .unwrap_or_else(|| {
                    self.mood_logic
                        .get(&Mood::Neutral)
                        .expect("Error: Neutral Mood is not implemented!")
                        .clone()
                });

        for (memory_key, cond, pool) in &self.known_ideas {
            if self.memory.ideas_said.contains(memory_key) && cond.is_met(&input_low) {
                return format!("{} {} {}", openers.pick(), pool.pick(), dismissals.pick());
            }
        }

        let matches: Vec<&str> = self
            .key_ideas
            .iter()
            .filter(|(cond, _)| cond.is_met(&input_low))
            .map(|(_, pool)| pool.pick())
            .collect();

        let middle = if matches.is_empty() {
            "...I don't even know what to say to that.".to_string()
        } else {
            matches.join(" ")
        };

        self.update_memory(&input_low);

        format!("{} {} {}", openers.pick(), middle, dismissals.pick())
    }
}

#[derive(Component, Default, Deref, Debug)]
pub struct Memory {
    pub ideas_said: HashSet<String>,
}
