use super::*;

static LOOK_UP: GlobalSignal<usize> = Signal::global(|| 0);

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub struct Uuid(usize);

impl Uuid {
    pub fn new() -> Self {
        let ret: usize = LOOK_UP();
        *LOOK_UP.write() += 1;
        Self(ret)
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::ops::Deref for Uuid {
    type Target = usize;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}