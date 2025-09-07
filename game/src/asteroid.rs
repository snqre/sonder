use super::*;

static SPRITE_URLS: [Asset; 5] = [
    asset!("asset/location/asteroid-0.gif"),
    asset!("asset/location/asteroid-1.gif"),
    asset!("asset/location/asteroid-2.gif"),
    asset!("asset/location/asteroid-3.gif"),
    asset!("asset/location/asteroid-4.gif")
];

#[derive(Debug)]
#[derive(Clone)]
pub struct Asteroid {
    addr: Address,
    population: Option<Address>,
    sprite_url: Asset
}

impl Asteroid {
    pub fn new() -> Service<Self> {
        let ret: Self = Self {
            addr: lock_env(|env| {
                env.claim_address()
            }),
            population: None,
            sprite_url: SPRITE_URLS[{
                let ret: usize = SPRITE_URLS.len();
                let ret: usize = ::fastrand::usize(0..ret);
                ret
            }]
        };
        (ret.addr, ret)
    }
}

impl ::engine::Sprite for Asteroid {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl ::engine::Service for Asteroid {
    type Event = Event;

    fn receive(&mut self, event: &Self::Event) -> engine::Effect<Self::Event> {
        match event {
            Event::Boot => ::engine::Effect::Batch(vec!(
                ::engine::Effect::Emit(vec!(Event::CelestialBodySpawn(Box::new({
                    self.to_owned()
                })))),
                ::engine::Effect::Emit(vec!(Event::AsteroidSpawn({
                    self.to_owned()
                }))),
                ::engine::Effect::Spawn({
                    let population_growth_multiplier: q::Q6<_> = (-1000_000000).into();
                    let (population_id, population) = Population::new(self.addr, 0, 0, population_growth_multiplier);
                    self.population = Some(population_id);
                    Box::new(population)
                })
            )),
            _ => ::engine::Effect::None
        }
    }
}

impl CelestialBody for Asteroid {}