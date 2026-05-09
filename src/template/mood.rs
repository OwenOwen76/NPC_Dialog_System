use crate::utility::math::WeightedPool;
use bevy::platform::collections::HashMap;
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use rand::prelude::SliceRandom;

#[derive(Default, Component, Eq, PartialEq, Hash, Debug, Clone)]
pub enum Mood {
    #[default]
    Neutral,
    Happy,
    Angry,
    Sad,
    Annoyed,
}

impl Mood {
    pub fn normal_pool() {
        let mut mood_logic = HashMap::new();

        mood_logic.insert(
            Mood::Happy,
            (
                WeightedPool {
                    variants: vec![
                        ("Hello, how ya doing?", 10),
                        ("Nice day, isn't it?", 8),
                        ("Hmmm... What should I do today?", 2),
                    ],
                },
                WeightedPool {
                    variants: vec![
                        ("Have a nice day.", 10),
                        ("Whacha gonna do today? I'm gonna go {}", 8),
                        ("Did you know that there's a {}", 5),
                    ],
                },
            ),
        );

        mood_logic.insert(
            Mood::Annoyed,
            (
                WeightedPool {
                    variants: vec![
                        ("Ugh.", 10),
                        ("What do you want now?", 5),
                        ("Make it quick, I'm busy.", 3),
                    ],
                },
                WeightedPool {
                    variants: vec![
                        ("Get lost.", 10),
                        ("Go bother someone else.", 5),
                        ("I'm done talking to you.", 2),
                    ],
                },
            ),
        );
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let moods = [
            Mood::Happy,
            Mood::Annoyed,
            Mood::Neutral,
            Mood::Angry,
            Mood::Sad,
        ];
        moods.choose(&mut rng).unwrap().clone()
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

#[cfg(test)]
mod tests {
    use crate::template::mood::Mood;

    #[test]
    fn random_mood_test() {
        let mut loop_amount = 10;
        loop {
            if loop_amount > 0 {
                let mood = Mood::random();
                println!("{:?}", mood);
                loop_amount -= 1;
            } else {
                break;
            }
        }
    }
}
