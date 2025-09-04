pub struct Bus<T> {
    event_handlers: Vec<Box<dyn FnMut(&T) -> Option<Vec<T>>>>
}

impl<T> Bus<T> {
    pub fn post(&mut self, event: T) {
        let mut queue: Vec<T> = vec!();
        for on_event in self.event_handlers.iter_mut() {
            if let Some(events) = on_event(&event) {
                for event in events {
                    queue.push(event);
                }
            }
        }
        for event in queue {
            self.post(event);
        }
    }

    pub fn on<A>(&mut self, on_event: A) 
    where
        A: FnMut(&T) -> Option<Vec<T>> + 'static {
        self.event_handlers.push(Box::new(on_event));
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