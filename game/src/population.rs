use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Population {
    id: ::engine::ServiceId,
    celestial_body: ::engine::ServiceId,
    growth_multiplier: q::Q6<i128>,
    count: u128
}

impl Population {
    pub fn new(
        celestial_body: ::engine::ServiceId,
        min_initial_count: u128,
        max_initial_count: u128,
        growth_multiplier: q::Q6<i128>
    ) -> ::engine::ServicePackage<Event> {
        let ret_id: ::engine::ServiceId = gen_service_id();
        let ret: Self = Self {
            id: gen_service_id(),
            celestial_body,
            growth_multiplier,
            count: ::fastrand::u128(min_initial_count..=max_initial_count)
        };
        (ret_id, ret).into()
    }

    pub fn celestial_body(&self) -> &::engine::ServiceId {
        &self.celestial_body
    }
}

impl ::engine::Identity for Population {
    fn id(&self) -> &::engine::ServiceId {
        &self.id
    }
}

impl ::engine::Service for Population {
    type Event = Event;

    fn receive(&mut self, event: &Self::Event) -> ::engine::Effect<Self::Event> {
        match event {
            Event::Boot => ::engine::Effect::Emit(vec!(Event::PopulationSpawn(*self))),
            Event::DayTermination(_) => {
                let count: i128 = self.count.try_into().unwrap();
                let count: q::Q6<i128> = (count * 1_000000).into();
                let count: q::Q6<i128> = (count * self.growth_multiplier).unwrap();
                let count: u128 = count.to_u128().unwrap();
                self.count = count;
                ::engine::Effect::None
            },
            _ => ::engine::Effect::None
        }
    }
}