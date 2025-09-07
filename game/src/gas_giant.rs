use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct GasGiant {
    id: ::engine::ServiceId,
    population: ::engine::ServiceId,
    sprite_url: Asset
}

impl ::engine::Sprite for GasGiant {
    fn sprite_url(&self) -> Asset {
        self.sprite_url
    }
}

impl ::engine::Identity for GasGiant {
    fn id(&self) -> &engine::ServiceId {
        &self.id
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
                    let id = self.id().to_owned();
                    let growth_multiplier: q::Q6<_> = (-1000_000000).into();
                    let (population_id, population) = Population::new(id, 0, 0, growth_multiplier).unpack();
                    self.population = population_id;
                    population
                })
            )),
            _ => ::engine::Effect::None
        }
    }
}

impl CelestialBody for GasGiant {}