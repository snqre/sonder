use super::*;

static SPRITE_URLS: [Asset; 3] = [
    asset!("asset/location/black-hole-0.gif"),
    asset!("asset/location/black-hole-1.gif"),
    asset!("asset/location/black-hole-2.gif")
];

pub struct BlackHole {
    sprite_url: Asset,
    name: name::Name,
    population: f64,
    population_change_rate: f64
}

impl BlackHole {
    pub fn new() -> Self {
        let choice: f64 = SPRITE_URLS.len() as f64 * ::fastrand::f64();
        let choice: usize = choice as usize;
        let sprite_url: Asset = SPRITE_URLS[choice];
        let name: name::Name = name::Name::random(name::Target::Asteroid);
        let population: f64 = 0.0;
        let population_change_rate: f64 = 0.0;
        Self {
            sprite_url,
            name,
            population,
            population_change_rate
        }
    }
}

impl common::Sprite for BlackHole {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl galaxy::CelestialBody for BlackHole {
    fn name(&self) -> name::Name {
        self.name.to_owned()
    }

    fn population(&self) -> f64 {
        self.population
    }

    fn population_change_rate(&self) -> f64 {
        self.population_change_rate
    }
}