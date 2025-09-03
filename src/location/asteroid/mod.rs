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
    population: population::Population
}

impl Asteroid {
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

impl galaxy::CelestialBody for Asteroid {
    fn spawn_multiplier(&self) -> f64 {
        2.0
    }

    fn name(&self) -> &name::Name {
        &self.name
    }

    fn population(&self) -> &population::Population {
        &self.population
    }
}

impl common::Sprite for Asteroid {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl common::Update for Asteroid {
    fn update(&mut self) {
        self.population.update();
    }
}