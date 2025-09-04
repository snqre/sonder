use super::*;

static ASTEROID_SPRITE_URLS: [Asset; 5] = [
    asset!("asset/location/asteroid-0.gif"),
    asset!("asset/location/asteroid-1.gif"),
    asset!("asset/location/asteroid-2.gif"),
    asset!("asset/location/asteroid-3.gif"),
    asset!("asset/location/asteroid-4.gif")
];

static BLACK_HOLE_SPRITE_URLS: [Asset; 3] = [
    asset!("asset/location/black-hole-0.gif"),
    asset!("asset/location/black-hole-1.gif"),
    asset!("asset/location/black-hole-2.gif")
];

static GAS_GIANT_SPRITE_URLS: [Asset; 1] = [
    asset!("asset/location/gas-giant-0.gif"),
];

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

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum AsteroidAdjective {
    Rocky,
    Jagged,
    Craggy,
    Dusty,
    Pitted,
    Shattered,
    Rugged,
    Scarred,
    Fragmented,
    Barren,
    Wandering,
    Ancient,
    Forgotten,
    Eternal,
    Lost,
    Ominous,
    Phantom,
    Hidden,
    Shrouded,
    Drifting
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum BlackHoleAdjective {
    Abyssal,
    Void,
    Shadowed,
    Infinite,
    Gravital,
    Devouring,
    Darkened,
    Singular,
    Crushing,
    Ominous
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum GalaxyAdjective {
    Spiral,
    Elliptical,
    Luminous,
    Expansive,
    Glittering,
    Infinite,
    Twirling,
    Majestic,
    Starry,
    Boundless
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum GasGiantAdjective {
    Massive,
    Swirling,
    Stormy,
    Gaseous,
    Colossaal,
    Turbulent,
    Banding,
    Roiling,
    Jovian,
    Tempestuous
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum IceWorldAdjective {
    Frozen,
    Glacial,
    Frostbitten,
    Icy,
    Snowy,
    Arctic,
    Frigid,
    Wintry,
    Chilled,
    Frosty
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum IslandsAdjective {
    Tropical,
    Archipelagic,
    Oceanic,
    Sunlit,
    Sandy,
    Coral,
    Verdant,
    Lagooned,
    Breezy,
    Isolated
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum LavaWorldAdjective {
    Blazing,
    Scorched,
    Molten,
    Searing,
    Fiery,
    Smoldering,
    Torrid,
    Incandescent,
    Volcanic,
    Sweltering
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum NoAtmosphereAdjective {
    Barren,
    Bleak,
    Exposed,
    Sterile,
    Rocky,
    Desolate,
    Craggy,
    Dusty,
    Windless
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum StarAdjective {
    Radiant,
    Blazing,
    Luminous,
    Fiery,
    Burning,
    Incandescent,
    Shimmering,
    Glowing,
    Brilliant,
    Pulsating
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum TerranDryAdjective {
    Arid,
    Parched,
    Dusty,
    Barren,
    Scorched,
    Rocky,
    Windswept,
    Sparse,
    Cracked
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum TerranWetAdjective {
    Humid,
    Lush,
    Verdant,
    Rainy,
    Misty,
    Swampy,
    Marshy,
    Fertile,
    Damp,
    Overflowing
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "title_case")]
enum Prefix {
    Ar,
    El,
    Io,
    Ua,
    Ae,
    Oe,
    Xy,
    Th,
    Kr,
    Zr,
    Vy,
    Gl,
    Qu,
    Fa,
    Ly,
    Na,
    Sa,
    Ta,
    Ze,
    Om
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
#[strum(serialize_all = "lowercase")]
enum Suffix {
    Ia,
    On,
    Ar,
    Or,
    Is,
    En,
    Us,
    Um,
    Ax,
    Ix,
    Os,
    Es,
    Al,
    Oria,
    Ura,
    Ion,
    Aes,
    Ora,
    Etha,
    Yth
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::EnumIter)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumIs)]
#[derive(::strum_macros::VariantArray)]
pub enum CelestialBody {
    Asteroid,
    BlackHole,
    GasGiant,
    IceWorld,
    Islands,
    LavaWorld,
    NoAtmosphere,
    Star,
    TerranDry,
    TerranWet
}

pub fn spawn_celestial_body(celestial_body: Option<CelestialBody>) {
    let m_origin: Address = Address::new_from_next();
    let m_celestial_body: CelestialBody = celestial_body.unwrap_or(random_celestial_body());
    let m_name: utf8::Utf8<64> = random_name(m_celestial_body).try_into().unwrap();
    let m_sprite_url: Asset = random_sprite_url(m_celestial_body);

    match m_celestial_body {
        CelestialBody::Asteroid => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 0,
            min_initial_count: 0,
            growth_multiplier: (-1000_000000).into()
        }),
        CelestialBody::BlackHole => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 0,
            min_initial_count: 0,
            growth_multiplier: (-1000_000000).into()
        }),
        CelestialBody::GasGiant => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 0,
            min_initial_count: 0,
            growth_multiplier: (-1000_000000).into()
        }),
        CelestialBody::IceWorld => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 3000000,
            min_initial_count: 0,
            growth_multiplier: (-1000_000000).into()
        }),
        CelestialBody::Islands => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 8000000000,
            min_initial_count: 1000000000,
            growth_multiplier: (-1000_000000).into()
        }),
        CelestialBody::LavaWorld => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 0,
            min_initial_count: 0,
            growth_multiplier: (-1000_000000).into()
        }),
        CelestialBody::NoAtmosphere => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 0,
            min_initial_count: 0,
            growth_multiplier: (-1000_000000).into()
        }),
        CelestialBody::Star => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 0,
            min_initial_count: 0,
            growth_multiplier: (-1000_000000).into()
        }),
        CelestialBody::TerranDry => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 8000000000,
            min_initial_count: 1000000000,
            growth_multiplier: 1_000005.into()
        }),
        CelestialBody::TerranWet => spawn_population(PopulationConfiguration {
            celestial_body: m_origin,
            max_initial_count: 900000000,
            min_initial_count: 200000000,
            growth_multiplier: 1_000006.into()
        })
    };

    super::Event::on(move |event| match event {
        super::Event::Boot => vec!(super::Event::CelestialBodySpawn {
            origin: m_origin,
            sprite_url: m_sprite_url,
            name: m_name.to_owned()
        }),
        _ => vec!()
    });
}

fn random_sprite_url(celestial_body: CelestialBody) -> Asset {
    match celestial_body {
        CelestialBody::Asteroid => {
            let ret: usize = ASTEROID_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = ASTEROID_SPRITE_URLS[ret];
            ret
        },
        CelestialBody::BlackHole => {
            let ret: usize = BLACK_HOLE_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = BLACK_HOLE_SPRITE_URLS[ret];
            ret
        },
        CelestialBody::GasGiant => {
            let ret: usize = GAS_GIANT_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = GAS_GIANT_SPRITE_URLS[ret];
            ret
        },
        CelestialBody::IceWorld => {
            let ret: usize = ICE_WORLD_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = ICE_WORLD_SPRITE_URLS[ret];
            ret
        },
        CelestialBody::Islands => {
            let ret: usize = ISLANDS_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = ISLANDS_SPRITE_URLS[ret];
            ret
        },
        CelestialBody::LavaWorld => {
            let ret: usize = LAVA_WORLD_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = LAVA_WORLD_SPRITE_URLS[ret];
            ret
        },
        CelestialBody::NoAtmosphere => {
            let ret: usize = NO_ATMOSPHERE_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = NO_ATMOSPHERE_SPRITE_URLS[ret];
            ret
        },
        CelestialBody::Star => {
            let ret: usize = STAR_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = STAR_SPRITE_URLS[ret];
            ret
        },
        CelestialBody::TerranDry => {
            let ret: usize = TERRAN_DRY_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = TERRAN_DRY_SPRITE_URLS[ret];
            ret
        },
        CelestialBody::TerranWet => {
            let ret: usize = TERRAN_WET_SPRITE_URLS.len();
            let ret: usize = ::fastrand::usize(0..ret);
            let ret: Asset = TERRAN_WET_SPRITE_URLS[ret];
            ret
        }
    }
}

fn random_celestial_body() -> CelestialBody {
    use ::strum::VariantArray as _;
    use ::strum::EnumCount as _;

    CelestialBody::VARIANTS[::fastrand::usize(0..CelestialBody::COUNT)]
}

fn random_name(celestial_body: CelestialBody) -> String {
    let adjective: String = random_name_adjective(celestial_body);
    let noun: String = random_name_prefix_and_suffix();
    let mut ret: String = String::new();
    ret.push_str(&adjective);
    ret.push(' ');
    ret.push_str(&noun);
    ret
}

fn random_name_adjective(celestial_body: CelestialBody) -> String {
    use ::strum::VariantArray as _;
    use ::strum::EnumCount as _;

    match celestial_body {
        CelestialBody::Asteroid => AsteroidAdjective::VARIANTS[::fastrand::usize(0..AsteroidAdjective::COUNT)].to_string(),
        CelestialBody::BlackHole => BlackHoleAdjective::VARIANTS[::fastrand::usize(0..BlackHoleAdjective::COUNT)].to_string(),
        CelestialBody::GasGiant => GasGiantAdjective::VARIANTS[::fastrand::usize(0..GasGiantAdjective::COUNT)].to_string(),
        CelestialBody::IceWorld => IceWorldAdjective::VARIANTS[::fastrand::usize(0..IceWorldAdjective::COUNT)].to_string(),
        CelestialBody::Islands => IslandsAdjective::VARIANTS[::fastrand::usize(0..IslandsAdjective::COUNT)].to_string(),
        CelestialBody::LavaWorld => LavaWorldAdjective::VARIANTS[::fastrand::usize(0..LavaWorldAdjective::COUNT)].to_string(),
        CelestialBody::NoAtmosphere => NoAtmosphereAdjective::VARIANTS[::fastrand::usize(0..NoAtmosphereAdjective::COUNT)].to_string(),
        CelestialBody::Star => StarAdjective::VARIANTS[::fastrand::usize(0..StarAdjective::COUNT)].to_string(),
        CelestialBody::TerranDry => TerranDryAdjective::VARIANTS[::fastrand::usize(0..TerranDryAdjective::COUNT)].to_string(),
        CelestialBody::TerranWet => TerranWetAdjective::VARIANTS[::fastrand::usize(0..TerranWetAdjective::COUNT)].to_string()
    }    
}

fn random_name_prefix_and_suffix() -> String {
    use ::strum::VariantArray as _;
    use ::strum::EnumCount as _;

    let prefix: String = Prefix::VARIANTS[::fastrand::usize(0..Prefix::COUNT)].to_string();
    let suffix: String = Suffix::VARIANTS[::fastrand::usize(0..Suffix::COUNT)].to_string();
    let mut ret: String = String::new();
    ret.push_str(&prefix);
    ret.push_str(&suffix);
    ret
}