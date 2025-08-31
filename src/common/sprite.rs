use super::*;

pub trait Sprite {
    fn sprite_url(&self) -> Asset;
}