pub trait Service {
    type Event;
    
    fn receive(&mut self, event: &Self::Event) -> Option<Vec<Self::Event>>;
}