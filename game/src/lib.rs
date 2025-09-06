#![deny(warnings)]

use ::dioxus::signals::GlobalSignal;

::modwire::expose!(
    pub logger
);

static GAME: GlobalSignal<::engine::Bus<Event>> = GlobalSignal::new(|| {
    ::engine::Bus::default()
});

#[derive(Debug)]
pub enum Event {
    Boot,
    DayTermination
}

pub fn post(event: Event) {
    GAME.write().post(event);
}

pub fn connect_boxed(service: Box<dyn ::engine::Service<Event = Event>>) -> ::engine::ServiceId {
    GAME.write().connect_boxed(service)
}

pub fn connect<T>(service: T) -> ::engine::ServiceId
where
    T: 'static,
    T: ::engine::Service<Event = Event> {
    GAME.write().connect(service)
}

pub fn disconnect(service_id: ::engine::ServiceId) {
    GAME.write().disconnect(service_id);
}