use super::*;

static SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/gas-giant-0.gif"),
];

pub struct GasGiant {
    sprite_url: Asset,
    name: name::Name,
    population: population::Population
}

impl GasGiant {
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

impl galaxy::CelestialBody for GasGiant {
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

impl common::Sprite for GasGiant {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl common::Update for GasGiant {
    fn update(&mut self) {
        
    }
}