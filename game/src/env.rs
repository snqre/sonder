use super::*;

static ENV: GlobalSignal<Env> = GlobalSignal::new(|| {
    Env {
        next_address: 0.into()
    }
});

pub trait Identity {
    fn address(&self) -> &Address;
}

pub type Service<T> = (Address, T);

pub struct Env {
    next_address: Address
}

impl Env {
    pub fn claim_address(&mut self) -> Address {
        *self.next_address += 1;
        self.next_address
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Address(u128);

impl From<u128> for Address {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl ops::Deref for Address {
    type Target = u128;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Address {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn lock_env<A, B>(on_lock: B) -> A 
where
    B: FnOnce(&mut Env) -> A {
    let mut driver: Write<'static, Env> = ENV.write();
    on_lock(&mut driver)
}