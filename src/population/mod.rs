use super::*;

pub struct Population {
    count: u128,
    growth_multiplier: f64
}

impl Population {
    pub fn new(count: u128, growth_multiplier: f64) -> Self {
        Self {
            count,
            growth_multiplier
        }
    }

    pub fn count(&self) -> u128 {
        self.count
    }

    pub fn growth_multiplier(&self) -> f64 {
        self.growth_multiplier
    }
}

impl common::Update for Population {
    fn update(&mut self) {
        if self.growth_multiplier == 0.0 {
            return
        }
        let count: f64 = self.count as f64;
        let count: f64 = count * self.growth_multiplier;
        let count: f64 = count.round().max(0.0);
        let count: u128 = count as u128;
        self.count = count;
    }
}