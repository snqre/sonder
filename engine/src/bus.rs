use super::*;

pub struct Bus<T> {
    services: Vec<Box<dyn Service<Event = T>>>
}

impl<A> Bus<A> {
    pub fn new() -> Self {
        let services: Vec<_> = vec!();
        Self {
            services
        }
    }

    pub fn post(&mut self, event: A) {
        let mut queue: Vec<_> = vec!();
        for service in self.services.iter_mut() {
            if let Some(events) = service.receive(&event) {
                for event in events {
                    queue.push(event);
                }
            }
        }
        for event in queue {
            self.post(event);
        }
    }
}