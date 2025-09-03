use super::*;

static COUNT: GlobalSignal<u128> = Signal::global(|| 0);

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub struct Address(u128);

impl Address {
    pub fn new(n: u128) -> Option<Self> {
        if *COUNT.read() >= n {
            return None
        }
        *COUNT.write() = n;
        Some(Self(n))
    }

    pub fn new_from_next() -> Self {
        *COUNT.write() += 1;
        let ret: u128 = *COUNT.read();
        let ret: Self = Self(ret);
        ret
    }
}

impl ::std::ops::Deref for Address {
    type Target = u128;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}