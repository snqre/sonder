use super::*;
use ::strum::EnumCount as _;
use ::strum::VariantArray as _;

static LOOK_UP: GlobalSignal<Vec<Name>> = Signal::global(|| {
    vec!()
});

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
pub enum Target {
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

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum Name {
    Asteroid(String),
    BlackHole(String),
    Galaxy(String),
    GasGiant(String),
    IceWorld(String),
    Islands(String),
    LavaWorld(String),
    NoAtmosphere(String),
    Star(String),
    TerranDry(String),
    TerranWet(String)
}

impl Name {
    pub fn new(target: Target) -> Self {
        for _ in 0..=64 {
            let adjective: String = Self::random_adjective(target);
            let noun: String = Self::random_prefix_and_suffix();
            let mut ret: String = String::new();
            ret.push_str(&adjective);
            ret.push(' ');
            ret.push_str(&noun);
            let ret: Self = match target {
                Target::Asteroid => Self::Asteroid(ret),
                Target::BlackHole => Self::BlackHole(ret),
                Target::Galaxy => Self::Galaxy(ret),
                Target::GasGiant => Self::GasGiant(ret),
                Target::IceWorld => Self::IceWorld(ret),
                Target::Islands => Self::Islands(ret),
                Target::LavaWorld => Self::LavaWorld(ret),
                Target::NoAtmosphere => Self::NoAtmosphere(ret),
                Target::Star => Self::Star(ret),
                Target::TerranDry => Self::TerranDry(ret),
                Target::TerranWet => Self::TerranWet(ret)
            };
            for name in LOOK_UP() {
                if ret != name {
                    LOOK_UP.write().push(
                        ret.to_owned()
                    );
                    return ret
                }
            }
        }
        let adjective: String = Self::random_adjective(target);
        let noun: String = Self::random_prefix_and_suffix();
        let mut ret: String = String::new();
        ret.push_str(&adjective);
        ret.push(' ');
        ret.push_str(&noun);
        let ret: Self = match target {
            Target::Asteroid => Self::Asteroid(ret),
            Target::BlackHole => Self::BlackHole(ret),
            Target::Galaxy => Self::Galaxy(ret),
            Target::GasGiant => Self::GasGiant(ret),
            Target::IceWorld => Self::IceWorld(ret),
            Target::Islands => Self::Islands(ret),
            Target::LavaWorld => Self::LavaWorld(ret),
            Target::NoAtmosphere => Self::NoAtmosphere(ret),
            Target::Star => Self::Star(ret),
            Target::TerranDry => Self::TerranDry(ret),
            Target::TerranWet => Self::TerranWet(ret)
        };
        LOOK_UP.write().push(
            ret.to_owned()
        );
        ret
    }

    fn random_adjective(target: Target) -> String {
        match target {
            Target::Asteroid => AsteroidAdjective::VARIANTS[::fastrand::usize(0..AsteroidAdjective::COUNT)].to_string(),
            Target::BlackHole => BlackHoleAdjective::VARIANTS[::fastrand::usize(0..BlackHoleAdjective::COUNT)].to_string(),
            Target::Galaxy => GalaxyAdjective::VARIANTS[::fastrand::usize(0..GalaxyAdjective::COUNT)].to_string(),
            Target::GasGiant => GasGiantAdjective::VARIANTS[::fastrand::usize(0..GasGiantAdjective::COUNT)].to_string(),
            Target::IceWorld => IceWorldAdjective::VARIANTS[::fastrand::usize(0..IceWorldAdjective::COUNT)].to_string(),
            Target::Islands => IslandsAdjective::VARIANTS[::fastrand::usize(0..IslandsAdjective::COUNT)].to_string(),
            Target::LavaWorld => LavaWorldAdjective::VARIANTS[::fastrand::usize(0..LavaWorldAdjective::COUNT)].to_string(),
            Target::NoAtmosphere => NoAtmosphereAdjective::VARIANTS[::fastrand::usize(0..NoAtmosphereAdjective::COUNT)].to_string(),
            Target::Star => StarAdjective::VARIANTS[::fastrand::usize(0..StarAdjective::COUNT)].to_string(),
            Target::TerranDry => TerranDryAdjective::VARIANTS[::fastrand::usize(0..TerranDryAdjective::COUNT)].to_string(),
            Target::TerranWet => TerranWetAdjective::VARIANTS[::fastrand::usize(0..TerranWetAdjective::COUNT)].to_string()
        }
    }

    fn random_prefix_and_suffix() -> String {
        let prefix: String = Prefix::VARIANTS[::fastrand::usize(0..Prefix::COUNT)].to_string();
        let suffix: String = Suffix::VARIANTS[::fastrand::usize(0..Suffix::COUNT)].to_string();
        let mut ret: String = String::new();
        ret.push_str(&prefix);
        ret.push_str(&suffix);
        ret
    }
}

impl ::std::ops::Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Asteroid(s)
            | Self::BlackHole(s)
            | Self::Galaxy(s)
            | Self::GasGiant(s)
            | Self::IceWorld(s)
            | Self::Islands(s) 
            | Self::LavaWorld(s) 
            | Self::NoAtmosphere(s)
            | Self::Star(s)
            | Self::TerranDry(s)
            | Self::TerranWet(s) => s
        }
    }
}