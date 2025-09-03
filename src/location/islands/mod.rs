use super::*;

static SPRITE_URLS: [Asset; 3] = [
    asset!("asset/location/islands-0.gif"),
    asset!("asset/location/islands-1.gif"),
    asset!("asset/location/islands-2.gif"),
];

pub struct Islands {
    sprite_url: Asset,
    name: name::Name,
    population: population::Population
}

impl Islands {
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

impl common::Sprite for Islands {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl common::Update for Islands {
    fn update(&mut self) {
        self.population.update();
    }
}