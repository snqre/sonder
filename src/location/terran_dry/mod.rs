use super::*;

static SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/terran-dry-0.gif"),
];

pub struct TerranDry {
    sprite_url: Asset,
    name: name::Name,
    population: Rbox<population::Population>
}

impl TerranDry {
    pub fn new() -> Self {
        let choice: f64 = SPRITE_URLS.len() as f64 * ::fastrand::f64();
        let choice: usize = choice as usize;
        let sprite_url: Asset = SPRITE_URLS[choice];
        let name: name::Name = name::Name::random(name::Target::Asteroid);
        let population: Rbox<_> = population::Population::new(0, -1000.0);
        Self {
            sprite_url,
            name,
            population
        }
    }
}

impl common::Sprite for TerranDry {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl galaxy::CelestialBody for TerranDry {
    fn spawn_multiplier(&self) -> f64 {
        1.8
    }

    fn name(&self) -> &name::Name {
        &self.name
    }

    fn population(&self) -> Rbox<population::Population> {
        self.population.to_owned()
    }
}