use super::*;
use location::*;
use strum::VariantArray;

static SPRITE_URLS: [Asset; 8] = [
    asset!("asset/location/galaxy-0.gif"),
    asset!("asset/location/galaxy-1.gif"),
    asset!("asset/location/galaxy-2.gif"),
    asset!("asset/location/galaxy-3.gif"),
    asset!("asset/location/galaxy-4.gif"),
    asset!("asset/location/galaxy-5.gif"),
    asset!("asset/location/galaxy-6.gif"),
    asset!("asset/location/galaxy-7.gif")
];

pub trait CelestialBody {
    /// `1.0` means there is `100%` chance this will spawn
    fn spawn_multiplier(&self) -> f64;
    fn name(&self) -> &name::Name;
    fn population(&self) -> &population::Population;
}

pub struct Galaxy {
    name: name::Name,
    sprite_url: Asset,
    celestial_bodies: Vec<Box<dyn CelestialBody>>
}

impl Galaxy {
    pub fn new() -> Self {
        let choice: f64 = SPRITE_URLS.len() as f64 * ::fastrand::f64();
        let choice: u16 = choice.round() as u16;
        let sprite_url: Asset = SPRITE_URLS[choice as usize];
        let name: name::Name = name::Name::random(name::Target::Galaxy);
        let celestial_bodies: Vec<Box<dyn CelestialBody>> = vec!();
        for _ in 0..=10 {
            let choice: f64 = name::Target::VARIANTS.len() as f64 * ::fastrand::f64();
            let choice: usize = choice.round() as usize;
            let target: name::Target = name::Target::VARIANTS[choice];
            match target {
                name::Target::Asteroid
            }
        }
        Self {
            name, 
            sprite_url,
            celestial_bodies
        }
    }
}

impl common::Sprite for Galaxy {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl common::Update for Galaxy {
    fn update(&mut self) {
        
    }
}