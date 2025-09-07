#![allow(unused_braces)]

use ::dioxus::signals::GlobalSignal;
use ::dioxus::prelude::Asset;
use ::dioxus::prelude::asset;
use ::dioxus::prelude::manganis;
use ::reliq::ops::ToPrim as _;
use ::reliq::q;
use ::engine::Identity as _;
use ::std::fmt;

::modwire::expose!(
    pub asteroid
    pub black_hole
    pub celestial_body
    pub day
    pub galaxy
    pub gas_giant
    pub logger
    pub population
);

static GAME: GlobalSignal<::engine::Bus<Event>> = GlobalSignal::new(|| {
    ::engine::Bus::default()
});

#[derive(Debug)]
pub enum Event {
    Boot,
    DayTermination(Day),
    CelestialBodySpawn(Box<dyn CelestialBody>),
    AsteroidSpawn(Asteroid),
    AsteroidMicroMeteorShower(Asteroid),
    AsteroidElectrostaticStorm(Asteroid),
    AsteroidSurfaceQuake(Asteroid),
    AsteroidCryovolcanicEruption(Asteroid),
    AsteroidMagneticAnomaly(Asteroid),
    AsteroidTemporalDistortion(Asteroid),
    BlackHoleSpawn(BlackHole),
    BlackHoleMatterTidalWave(BlackHole),
    BlackHoleAccretionDiskStorm(BlackHole),
    BlackHoleGravitationalLensingFlare(BlackHole),
    BlackHoleParallelGlimpse(BlackHole),
    BlackHoleLivingSingularity(BlackHole),
    BlackHoleHullStressCrisis(BlackHole),
    BlackHoleEscapeBurn(BlackHole),
    GasGiantSpawn(GasGiant),
    GasGiantStormMigration(GasGiant),
    GasGiantAtmosphericUpdraft(GasGiant),
    GasGiantPressureShockwave(GasGiant),
    GasGiantMagnetosphereFlare(GasGiant),

    CelestialBodyIslandsVolcanicEruption,
    CelestialBodyIslandsTidalFlood,
    CelestialBodyIslandsHurricane,
    CelestialBodyIslandsHiddenCaveSystem,

    // an island is actually aa massive creature
    CelestialBodyIslandsMovingIsland,
    // .. follow up events trigger on play decision

    PopulationSpawn(Population)
}

pub fn post(event: Event) {
    GAME.write().post(event);
}

pub fn connect_package<T>(package: T)
where
    T: Into<::engine::ServicePackage<Event>> {
    GAME.write().connect_package({
        package.into()
    });
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

pub fn gen_service_id() -> ::engine::ServiceId {
    GAME.write().gen_service_id()
}