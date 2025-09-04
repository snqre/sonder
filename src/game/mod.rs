use super::GlobalSignal;
use super::Writable as _;
use super::Readable as _;
use super::engine;
use super::q;

::modwire::expose!(
    pub logger
    pub population
);

pub static BUS: GlobalSignal<engine::Bus<Event>> = GlobalSignal::new(|| {
    engine::Bus::default()
});

#[derive(Debug)]
#[derive(Clone)]
pub enum Event {
    Boot,
    Tick,
    PopulationUpdate(Population),
    CelestialBodyUpdate(),
    MarketUpdate()
}

pub fn post(event: Event) {
    BUS.write().post(event);
}

pub fn connect<T>(node: T)
where
    T: engine::Node<Event = Event> + 'static {
    BUS.write().connect(node);
}