use super::GlobalSignal;
use super::Writable as _;
use super::Readable as _;
use super::engine;
use super::q;
use super::utf8;
use super::map;
use super::array;

pub mod component;
pub mod rule;

pub static ENGINE: GlobalSignal<engine::Engine<Component>> = GlobalSignal::new(|| {
    engine::Engine::new()
        .add_component(component::Item::new("", ""))
        .add_component(component::Population::new(2000, 20, 1_000001.into()))
        .add_rule(rule::PopulationGrowth)
        .build()
});

pub enum Component {
    CelestialBody(CelestialBody),
    Item(Item),
    Population(Population)
}

impl From<Population> for Component {
    fn from(value: Population) -> Self {
        Component::Population(value)
    }
}

impl From<Item> for Component {
    fn from(value: Item) -> Self {
        Component::Item(value)
    }
}