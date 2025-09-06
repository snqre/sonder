use super::*;

pub trait Service {
    type Event;
    
    fn receive(&mut self, event: &Self::Event) -> Effect<Self::Event>;
}