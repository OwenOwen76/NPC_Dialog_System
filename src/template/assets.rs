use bevy::platform::collections::HashMap;
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy::state::state::States;
use rand::prelude::SliceRandom;

#[derive(Resource, Debug, Deref, Default)]
pub struct Locations {
    pub normal_locations: HashSet<&'static str>,
}

#[derive(Resource, Debug, Deref, Default)]
pub struct Actions {
    pub everyday_actions: HashSet<&'static str>,
}

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Stormy,
}

impl Weather {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let weathers = [
            Weather::Sunny,
            Weather::Cloudy,
            Weather::Rainy,
            Weather::Stormy,
        ];
        weathers.choose(&mut rng).unwrap().clone()
    }
}
