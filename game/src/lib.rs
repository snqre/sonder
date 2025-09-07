#![allow(unused_braces)]

use ::dioxus::prelude::Write;
use ::dioxus::signals::GlobalSignal;
use ::dioxus::prelude::Asset;
use ::dioxus::prelude::asset;
use ::dioxus::prelude::manganis;
use ::reliq::ops::ToPrim as _;
use ::reliq::q;
use ::engine::Identity as _;
use ::std::fmt;
use ::std::ops;

::modwire::expose!(
    pub asteroid
    pub black_hole
    pub celestial_body
    pub day
    pub env
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

// DO NOT GRAB A LOCK ON THE BUS WITHIN A SERVICE OR THE LOCK ITSELF!
pub fn lock<A, B>(on_lock: B) -> A 
where
    B: FnOnce(&mut ::engine::Bus<Event>) -> A {
    let mut bus: Write<'static, ::engine::Bus<Event>> = GAME.write();
    on_lock(&mut bus)
}