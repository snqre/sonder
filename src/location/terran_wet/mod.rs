use super::*;

static SPRITE_URLS: [Asset; 3] = [
    asset!("asset/location/terran-wet-0.gif"),
    asset!("asset/location/terran-wet-1.gif"),
    asset!("asset/location/terran-wet-2.gif")
];

pub struct TerranWet {
    sprite_url: Asset,
    name: name::Name,
    population: population::Population
}

impl TerranWet {
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

impl galaxy::CelestialBody for TerranWet {
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

impl common::Sprite for TerranWet {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl common::Update for TerranWet {
    fn update(&mut self) {
        self.population.update();
    }
}