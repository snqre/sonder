use super::*;

static SPRITE_URLS: [Asset; 3] = [
    asset!("asset/location/islands-0.gif"),
    asset!("asset/location/islands-1.gif"),
    asset!("asset/location/islands-2.gif"),
];

pub struct Islands {
    sprite_url: Asset,
    name: name::Name,
    population: Box<population::Population>
}

impl Islands {
    pub fn new() -> Self {
        let choice: f64 = SPRITE_URLS.len() as f64 * ::fastrand::f64();
        let choice: usize = choice as usize;
        let sprite_url: Asset = SPRITE_URLS[choice];
        let name: name::Name = name::Name::random(name::Target::Asteroid);
        let population: Box<_> = population::Population::new(::fastrand::u128(1_000_000_000..=8_000_000_000), 1.0025);
        Self {
            sprite_url,
            name,
            population
        }
    }
}

impl common::Sprite for Islands {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl galaxy::CelestialBody for Islands {
    fn spawn_multiplier(&self) -> f64 {
        1.5
    }

    fn name(&self) -> &name::Name {
        &self.name
    }

    fn population(&self) -> &population::Population {
        &self.population
    }
}