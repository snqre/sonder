use super::*;

pub struct Bus<T> {
    services: Vec<(ServiceId, Box<dyn Service<Event = T>>)>,
    next_service_id: ServiceId
}

impl<A> Bus<A> {
    pub fn new() -> Self {
        let services: Vec<_> = vec!();
        Self {
            services,
            next_service_id: 0.into()
        }
    }

    pub fn post(&mut self, event: A) {
        let mut queue: ds::VecDeque<_> = vec!(event).into();
        while let Some(event) = queue.pop_front() {
            let mut kill_queue: Vec<_> = vec!();
            let mut spawn_queue: Vec<_> = vec!();
            for (service_id, service) in self.services.iter_mut() {
                let effect: Effect<_> = service.receive(&event);
                match effect {
                    Effect::None => {},
                    Effect::Kill => {
                        let service_id: ServiceId = service_id.to_owned();
                        kill_queue.push(service_id);
                    },
                    Effect::Emit(events) => queue.extend(events),
                    Effect::Defer(event) => queue.push_back(event),
                    Effect::Spawn(service) => spawn_queue.push(service),
                    Effect::Batch(batch) => {
                        for effect in batch {
                            let events = effect.flatten();
                            queue.extend(events);
                        }
                    }
                };
            }
            for service_id in kill_queue {
                self.disconnect(service_id);
            }
            for service in spawn_queue {
                self.connect_boxed(service);
            }
        }
    }

    pub fn connect_boxed(&mut self, service: Box<dyn Service<Event = A>>) -> ServiceId {
        let service_id: ServiceId = self.gen_service_id();
        self.services.push((service_id, service));
        service_id
    }

    pub fn connect<B>(&mut self, service: B) -> ServiceId
    where
        B: 'static,
        B: Service<Event = A> {
        let service_id: ServiceId = self.gen_service_id();
        let service: Box<_> = Box::new(service);
        self.services.push((service_id, service));
        service_id
    }

    pub fn disconnect(&mut self, service_id: ServiceId) {
        self.services.retain(|(id, _)| {
            *id != service_id
        });
    }

    fn gen_service_id(&mut self) -> ServiceId {
        let ret: ServiceId = self.next_service_id;
        let one: ServiceId = 1.into();
        self.next_service_id = (*self.next_service_id + *one).into();
        ret
    }
}