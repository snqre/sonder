use super::*;

static ASTEROID_SPRITE_URLS: [Asset; 5] = [
    asset!("asset/location/asteroid-0.gif"),
    asset!("asset/location/asteroid-1.gif"),
    asset!("asset/location/asteroid-2.gif"),
    asset!("asset/location/asteroid-3.gif"),
    asset!("asset/location/asteroid-4.gif")
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
    Galaxy,
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
    let m_port: u128 = next();
    let m_celestial_body: CelestialBody = celestial_body.unwrap_or(random_celestial_body());
    let m_name: String = random_name(m_celestial_body);

    spawn_population(
        m_port,
        0,
        50000,
        1_01.into()
    );

    on(Box::new(move |event| {
        match event {
            super::Event::Boot => post(Event::CelestialBodySpawn {
                port: m_port,
                name: m_name.to_string()
            }),
            _ => {}
        }
    }));
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
        CelestialBody::Galaxy => GalaxyAdjective::VARIANTS[::fastrand::usize(0..GalaxyAdjective::COUNT)].to_string(),
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

