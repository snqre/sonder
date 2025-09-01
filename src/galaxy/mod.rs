use super::*;
use ::strum::VariantArray;
use location::*;

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

pub trait CelestialBody 
where
    Self: common::Update {
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
        let name: name::Name = name::Name::new(name::Target::Galaxy);
        let mut celestial_bodies: Vec<Box<dyn CelestialBody>> = vec!();
        for _ in 0..=10 {
            let choice: f64 = name::Target::VARIANTS.len() as f64 * ::fastrand::f64();
            let choice: usize = choice.round() as usize;
            let target: name::Target = name::Target::VARIANTS[choice];
            let celestial_body: Box<dyn CelestialBody> = match target {
                name::Target::Asteroid => Box::new(asteroid::Asteroid::new()),
                name::Target::BlackHole | name::Target::Galaxy => Box::new(black_hole::BlackHole::new()),
                name::Target::GasGiant => Box::new(gas_giant::GasGiant::new()),
                name::Target::IceWorld => Box::new(ice_world::IceWorld::new()),
                name::Target::Islands => Box::new(islands::Islands::new()),
                name::Target::LavaWorld => Box::new(lava_world::LavaWorld::new()),
                name::Target::NoAtmosphere => Box::new(no_atmosphere::NoAtmosphere::new()),
                name::Target::Star => Box::new(star::Star::new()),
                name::Target::TerranDry => Box::new(terran_dry::TerranDry::new()),
                name::Target::TerranWet => Box::new(terran_wet::TerranWet::new())
            };
            celestial_bodies.push(celestial_body);
        }
        Self {
            name, 
            sprite_url,
            celestial_bodies
        }
    }

    pub fn name(&self) -> &name::Name {
        &self.name
    }

    pub fn celestial_bodies(&self) -> &Vec<Box<dyn CelestialBody>> {
        &self.celestial_bodies
    }
}

impl common::Sprite for Galaxy {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl common::Update for Galaxy {
    fn update(&mut self) {
        for celestial_body in self.celestial_bodies.iter_mut() {
            celestial_body.update();
        }
    }
}