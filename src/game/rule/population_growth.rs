use super::*;

pub struct PopulationGrowth;

impl engine::Rule for PopulationGrowth {
    type Component = Component;

    fn apply(&self, components: &mut Vec::<Self::Component>) {
        for component in components {
            if let Component::Population(population) = component {
                population.grow_or_decline();
            }
        }
    }
}