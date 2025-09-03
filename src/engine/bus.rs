use super::*;

pub struct Bus<T> {
    event_handlers: Vec<Box<dyn FnMut(&T) -> Vec<T>>>
}

impl<T> Bus<T> {
    pub fn post(&mut self, event: T) {
        let mut events: Vec<T> = vec!();
        for on_event in self.event_handlers.iter_mut() {
            let additional_events = on_event(&event);
            for event in additional_events {
                events.push(event);
            }
        }
        for event in events {
            self.post(event);
        }
    }

    pub fn on<A>(&mut self, on_event: A) 
    where
        A: FnMut(&T) -> Vec<T> + 'static {
        self.event_handlers.push(Box::new(on_event));
    }

    pub fn filter() {

    }
}

impl<T> Default for Bus<T> {
    fn default() -> Self {
        let event_handlers: Vec<_> = vec!();
        Self {
            event_handlers
        }
    }
}