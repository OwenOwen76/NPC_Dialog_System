use crate::idea;
use crate::template::{mood::Mood, template::*};
use crate::utility::{
    input::{KeyCondition, KeyCondition::*, UserInput},
    math::WeightedPool,
    typing_effect::*,
};
use bevy::platform::collections::HashMap;
use std::io::{self, Write};

pub fn build_girl() -> Npc {
    let mut mood_logic = HashMap::new();
    let mut key_ideas: Vec<(KeyCondition, WeightedPool)> = Vec::new();
    let mut known_ideas: Vec<(String, KeyCondition, WeightedPool)> = Vec::new();

    // MOOD
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
                    ("Now get lost.", 10),
                    ("Go bother someone else.", 5),
                    ("I'm done talking to you.", 2),
                ],
            },
        ),
    );

    mood_logic.insert(
        Mood::Neutral,
        (
            WeightedPool {
                variants: vec![("Fine. Speak.", 1)],
            },
            WeightedPool {
                variants: vec![("Whatever.", 1)],
            },
        ),
    );

    // KEY IDEAS
    key_ideas = vec![
        idea!(
            (And(vec![Word("who"), Word("are"), Word("you"),])),
            ("I already told you to mind your business.", 1)
        ),
        idea!(
            "help",
            ("Help? You're the hero. Figure it out yourself.", 1)
        ),
        idea!(
            "where",
            ("You're right here, and that all that matters.", 1)
        ),
        idea!(
            "hi",
            ("What is it?", 3),
            ("Speak.", 5),
            ("Talk if you must", 1)
        ),
    ];

    known_ideas = vec![
        idea!(
            "who are you" => (And(vec![Word("who"), Word("are"), Word("you")])),
            ("I already told you to mind your business! Stop asking.", 1)
        ),
        idea!(
            "help" => "help", ("Help again? I've already told you to figure it out yourself", 1)
        ),
        idea!(
            "where" => "where", ("Ugh... Didn't I just say that it doesn't matter right now? You'll find out when the time is right.", 1)
        ),
    ];

    Npc::new(
        "???".to_string(),
        mood_logic,
        known_ideas,
        key_ideas,
        Memory::default(),
    )
}

pub fn run_girl() {
    let mut the_girl = build_girl();
    let mut current_mood = Mood::Annoyed;
    let mut input_buffer = UserInput::default();

    println!();
    println!("???: What? Make it quick.");

    loop {
        input_buffer.clear();

        print!("\nYou: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line");

        let trimmed = input_buffer.trim();

        let exit_words = ["bye", "leav", "quit"];

        if exit_words.iter().any(|&word| trimmed.contains(word)) {
            print!("???: ");
            typewriter_println("Fine. Leave. See if I care.");
            break;
        }

        let response = the_girl.respond(&current_mood, trimmed);

        print!("\n{}: ", *the_girl);
        typewriter_println(&response);

        the_girl.print_debug();

        if trimmed.contains("sorry") {
            current_mood = Mood::Neutral;
        }
    }
}
