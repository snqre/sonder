pub trait Service {
    type Event;
    
    fn receive(&mut self, event: &Self::Event) -> Option<Vec<Self::Event>>;
}

pub struct Bus<T> {
    services: Vec<Box<dyn Service<Event = T>>>,
}

impl<A> Bus<A> {
    pub fn new() -> Self {
        Self {
            services: vec!()
        }
    }

    pub fn post(&mut self, event: A) {
        let mut queue: Vec<_> = vec!();
        for service in (*self).services.iter_mut() {
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

    pub fn connect<B>(&mut self, service: B)
    where
        B: Service<Event = A> + 'static {
        self.services.push(Box::new(service));
    }
}

impl<T> Default for Bus<T> 
where 
    T: Default {
    fn default() -> Self {
        let services: Vec<_> = vec!();
        Self {
            services
        }
    }
}