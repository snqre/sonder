use super::*;

pub trait Identity {
    fn id(&self) -> &ServiceId;
}