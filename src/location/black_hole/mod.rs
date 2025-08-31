use super::*;

static SPRITE_URLS: [Asset; 3] = [
    asset!("asset/location/black-hole-0.gif"),
    asset!("asset/location/black-hole-1.gif"),
    asset!("asset/location/black-hole-2.gif")
];

pub struct BlackHole {
    sprite_url: Asset,
    name: name::Name,
    population: Rbox<population::Population>
}

impl BlackHole {
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

impl common::Sprite for BlackHole {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl galaxy::CelestialBody for BlackHole {
    fn spawn_multiplier(&self) -> f64 {
        0.9
    }

    fn name(&self) -> &name::Name {
        &self.name
    }

    fn population(&self) -> Rbox<population::Population> {
        self.population.to_owned()
    }
}