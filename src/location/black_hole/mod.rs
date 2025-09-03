use super::*;

static SPRITE_URLS: [Asset; 3] = [
    asset!("asset/location/black-hole-0.gif"),
    asset!("asset/location/black-hole-1.gif"),
    asset!("asset/location/black-hole-2.gif")
];

pub struct BlackHole {
    sprite_url: Asset,
    name: name::Name,
    population: population::Population
}

impl BlackHole {
    pub fn new() -> Self {
        let sprite_urls_len: usize = SPRITE_URLS.len();
        let sprite_url: Asset = SPRITE_URLS[::fastrand::usize(0..sprite_urls_len)];
        let name: name::Name = name::Name::new(name::Target::Asteroid);
        let population: population::Population = population::Population::new(0, -1000.0);
        Self {
            sprite_url,
            name,
            population
        }
    }
}

impl galaxy::CelestialBody for BlackHole {
    fn spawn_multiplier(&self) -> f64 {
        0.9
    }

    fn name(&self) -> &name::Name {
        &self.name
    }

    fn population(&self) -> &population::Population {
        &self.population
    }
}

impl common::Sprite for BlackHole {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl common::Update for BlackHole {
    fn update(&mut self) {
        
    }
}