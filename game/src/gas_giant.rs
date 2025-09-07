use super::*;

static SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/gas-giant-0.gif"),
];

#[derive(Debug)]
#[derive(Clone)]
pub struct GasGiant {
    address: Address,
    population: Option<Address>,
    sprite_url: Asset
}

impl Identity for GasGiant {
    fn address(&self) -> &Address {
        &self.address
    }
}

impl ::engine::Sprite for GasGiant {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl ::engine::Service for GasGiant {
    type Event = Event;

    fn receive(&mut self, event: &Self::Event) -> engine::Effect<Self::Event> {
        match event {
            Event::Boot => ::engine::Effect::Batch(vec!(
                ::engine::Effect::Emit(vec!(Event::GasGiantSpawn({
                    self.to_owned()
                }))),
                ::engine::Effect::Spawn({
                    let growth_multiplier: q::Q6<_> = (-1000_000000).into();
                    let (ret_address, ret) = Population::new(self.address, 0, 0, growth_multiplier);
                    let ret: Box<_> = Box::new(ret);
                    self.population = Some(ret_address);
                    ret
                })
            )),
            _ => ::engine::Effect::None
        }
    }
}

impl CelestialBody for GasGiant {}