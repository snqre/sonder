use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Population {
    addr: Address,
    celestial_body: Address,
    growth_multiplier: q::Q6<i128>,
    count: u128
}

impl Population {
    pub fn new(
        celestial_body: Address,
        min_initial_count: u128,
        max_initial_count: u128,
        growth_multiplier: q::Q6<i128>
    ) -> Service<Self> {
        let ret: Self = Self {
            addr: lock_env(|env| {
                env.claim_address()
            }),
            celestial_body,
            growth_multiplier,
            count: ::fastrand::u128(min_initial_count..=max_initial_count)
        };
        (ret.addr, ret)
    }

    pub fn celestial_body(&self) -> &Address {
        &self.celestial_body
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