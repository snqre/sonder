use super::*;

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct ServiceId(u32);

impl From<u32> for ServiceId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl ops::Deref for ServiceId {
    type Target = u32;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ServiceId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}