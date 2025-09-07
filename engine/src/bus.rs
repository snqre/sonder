use super::*;

pub struct Bus<T> {
    services: Vec<(ServiceId, Box<dyn Service<Event = T>>)>,
    service_count: ServiceId
}

impl<A> Bus<A> {
    pub fn post(&mut self, event: A) {
        let mut queue: Vec<_> = vec!();
        let mut deletion_queue: Vec<_> = vec!();
        let mut spawn_queue: Vec<_> = vec!();
        for (service_id, service) in self.services.iter_mut() {
            let effect: Effect<_> = service.receive(&event);
            match effect {
                Effect::None => {},
                Effect::Kill => {
                    let service_id: ServiceId = service_id.to_owned();
                    deletion_queue.push(service_id);
                },
                Effect::Emit(events) => queue.extend(events),
                Effect::Defer(event) => queue.push(event),
                Effect::Spawn(service) => spawn_queue.push(service),
                Effect::Batch(batch) => {
                    for effect in batch {
                        let events = effect.flatten();
                        queue.extend(events);
                    }
                }
            }
        }
        for service_id in deletion_queue {
            self.disconnect(service_id);
        }
        for service in spawn_queue {
            self.connect_boxed(service);
        }
        for event in queue {
            self.post(event);
        }        
    }

    pub fn connect_package<B>(&mut self, package: B) -> ServiceId
    where
        B: Into<ServicePackage<A>> {
        let package: ServicePackage<A> = package.into();
        let (service_id, service) = package.unpack();
        self.services.push((service_id, service));
        service_id
    }

    pub fn connect_boxed(&mut self, service: Box<dyn Service<Event = A>>) -> ServiceId {
        let service_id: ServiceId = self.generate_service_id();
        self.services.push((service_id, service));
        service_id
    }

    pub fn connect<B>(&mut self, service: B) -> ServiceId
    where
        B: 'static,
        B: Service<Event = A> {
        let service_id: ServiceId = self.generate_service_id();
        let service: Box<_> = Box::new(service);
        self.services.push((service_id, service));
        service_id
    }

    pub fn disconnect(&mut self, service_id: ServiceId) {
        self.services.retain(|(id, _)| {
            *id != service_id
        });
    }

    pub fn generate_service_id(&mut self) -> ServiceId {
        let ret: ServiceId = self.service_count;
        let one: ServiceId = 1.into();
        self.service_count = (*self.service_count + *one).into();
        ret
    }
}

impl<T> Default for Bus<T> {
    fn default() -> Self {
        let services: Vec<_> = vec!();
        Self {
            services,
            service_count: 0.into()
        }
    }
}