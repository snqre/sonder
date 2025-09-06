use super::*;

#[derive(Debug)]
pub struct Population {
    growth_multiplier: q::Q6<i128>,
    count: u128
}

impl Population {
    pub fn new(
        max_initial_count: u128,
        min_initial_count: u128,
        growth_multiplier: q::Q6<i128>
    ) -> Self {
        Self {
            growth_multiplier,
            count: ::fastrand::u128(min_initial_count..=max_initial_count)
        }
    }

    pub fn growth_multiplier(&self) -> &q::Q6<i128> {
        &self.growth_multiplier
    }

    pub fn count(&self) -> &u128 {
        &self.count
    }

    pub fn grow_or_decline(&mut self) {
        use ::reliq::ops::ToPrim as _;
        let count: i128 = self.count.try_into().unwrap();
        let count: q::Q6<i128> = (count * 1_000000).into();
        let count: q::Q6<i128> = (count * self.growth_multiplier).unwrap();
        let count: u128 = count.to_u128().unwrap();
        self.count = count;
    }
}