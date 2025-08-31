use super::*;

static SPRITE_URLS: [Asset; 5] = [
    asset!("asset/location/asteroid-0.gif"),
    asset!("asset/location/asteroid-1.gif"),
    asset!("asset/location/asteroid-2.gif"),
    asset!("asset/location/asteroid-3.gif"),
    asset!("asset/location/asteroid-4.gif")
];

pub struct Asteroid {
    sprite_url: Asset,
    name: name::Name,
    population: f64,
    population_change_rate: f64
}

impl Asteroid {
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

impl common::Sprite for Asteroid {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl galaxy::CelestialBody for Asteroid {
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