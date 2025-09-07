use super::*;

pub trait CelestialBody 
where
    Self: fmt::Debug {

}

static ICE_WORLD_SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/ice-world-0.gif"),
];

static ISLANDS_SPRITE_URLS: [Asset; 3] = [
    asset!("asset/location/islands-0.gif"),
    asset!("asset/location/islands-1.gif"),
    asset!("asset/location/islands-2.gif"),
];

static LAVA_WORLD_SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/lava-world-0.gif"),
];

static NO_ATMOSPHERE_SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/no-atmosphere-0.gif"),
];

static STAR_SPRITE_URLS: [Asset; 4] = [
    asset!("asset/location/star-0.gif"),
    asset!("asset/location/star-1.gif"),
    asset!("asset/location/star-2.gif"),
    asset!("asset/location/star-3.gif")
];

static TERRAN_DRY_SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/terran-dry-0.gif"),
];

static TERRAN_WET_SPRITE_URLS: [Asset; 3] = [
    asset!("asset/location/terran-wet-0.gif"),
    asset!("asset/location/terran-wet-1.gif"),
    asset!("asset/location/terran-wet-2.gif")
];