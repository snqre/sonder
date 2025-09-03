use super::*;

static SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/ice-world-0.gif"),
];

pub struct IceWorld {
    sprite_url: Asset,
    name: name::Name,
    population: population::Population
}

impl IceWorld {
    pub fn new() -> Self {
        let sprite_urls_len: usize = SPRITE_URLS.len();
        let sprite_url: Asset = SPRITE_URLS[::fastrand::usize(0..sprite_urls_len)];
        let name: name::Name = name::Name::new(name::Target::Asteroid);
        let population: population::Population = population::Population::new(::fastrand::u128(1_000_000_000..=8_000_000_000), 1.0025);
        Self {
            sprite_url,
            name,
            population
        }
    }
}

impl galaxy::CelestialBody for IceWorld {
    fn spawn_multiplier(&self) -> f64 {
        0.7
    }

    fn name(&self) -> &name::Name {
        &self.name
    }

    fn population(&self) -> &population::Population {
        &self.population
    }
}

impl common::Sprite for IceWorld {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl common::Update for IceWorld {
    fn update(&mut self) {
        self.population.update();
    }
}