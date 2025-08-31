use super::*;

static SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/no-atmosphere-0.gif"),
];

pub struct NoAtmosphere {
    sprite_url: Asset,
    name: name::Name,
    population: Box<population::Population>
}

impl NoAtmosphere {
    pub fn new() -> Self {
        let choice: f64 = SPRITE_URLS.len() as f64 * ::fastrand::f64();
        let choice: usize = choice as usize;
        let sprite_url: Asset = SPRITE_URLS[choice];
        let name: name::Name = name::Name::random(name::Target::Asteroid);
        let population: Box<_> = population::Population::new(0, -1000.0);
        Self {
            sprite_url,
            name,
            population
        }
    }
}

impl common::Sprite for NoAtmosphere {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl galaxy::CelestialBody for NoAtmosphere {
    fn spawn_multiplier(&self) -> f64 {
        1.0
    }

    fn name(&self) -> &name::Name {
        &self.name
    }

    fn population(&self) -> &population::Population {
        &self.population
    }
}