use super::*;

pub struct ServicePackage<T> {
    service_id: ServiceId,
    service: Box<dyn Service<Event = T>>
}

impl<T> ServicePackage<T> {
    pub fn unpack(self) -> (ServiceId, Box<dyn Service<Event = T>>) {
        let a: ServiceId = self.service_id;
        let b: Box<dyn Service<Event = T>> = self.service;
        (a, b)
    }
}

impl<A, B, C> From<(A, B)> for ServicePackage<C>
where
    A: Into<ServiceId>,
    B: 'static,
    B: Service<Event = C> {
    fn from(value: (A, B)) -> Self {
        let service_id: ServiceId = value.0.into();
        let service: B = value.1;
        let service: Box<dyn Service<Event = C>> = Box::new(service);
        Self {
            service_id,
            service
        }
    }
}