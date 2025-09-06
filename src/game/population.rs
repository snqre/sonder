use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct Population {
    origin: engine::Address,
    celestial_body: engine::Address,
    max_initial_count: u128,
    min_initial_count: u128,
    growth_multiplier: q::Q6<i128>,
    count: u128
}

impl Population {
    pub fn new(
        celestial_body: engine::Address,
        max_initial_count: u128,
        min_initial_count: u128,
        growth_multiplier: q::Q6<i128>
    ) -> Self {
        Self {
            origin: engine::Address::new_from_next(),
            celestial_body,
            max_initial_count,
            min_initial_count,
            growth_multiplier,
            count: ::fastrand::u128(min_initial_count..=max_initial_count)
        }
    }
}

impl engine::Service for Population {
    type Event = Event;

    fn receive(&mut self, event: &Self::Event) -> Option<Vec<Self::Event>> {
        use ::reliq::ops::ToPrim as _;
        match event {
            Event::Boot => Some(vec!(
                Event::PopulationSpawn(self.to_owned()),
                Event::PopulationUpdate(self.to_owned())
            )),
            Event::Tick => {
                let count: i128 = self.count.try_into().unwrap();
                let count: q::Q6<i128> = (count * 1_000000).into();
                let count: q::Q6<i128> = (count * self.growth_multiplier).unwrap();
                let count: u128 = count.to_u128().unwrap();
                self.count = count;
                Some(vec!(Event::PopulationUpdate(self.to_owned())))
            },
            _ => None
        }
    }
}