use super::*;

pub struct Item {
    addr: engine::Address,
    name: utf8::Utf8<64>,
    description: utf8::Utf8<256>,
    total_supply: q::Q2<u128>,
    address_to_balance: Box<map::Map<256, engine::Address, q::Q2<u128>>>
}

impl Item {
    pub fn new<A, B>(name: A, description: B) -> Self 
    where
        A: TryInto<utf8::Utf8<64>>,
        B: TryInto<utf8::Utf8<256>> {
        let name: utf8::Utf8<64> = name.try_into().ok().unwrap();
        let description: utf8::Utf8<256> = description.try_into().ok().unwrap();
        Self {
            addr: engine::Address::new_from_next(),
            name,
            description,
            total_supply: q::as_0(),
            address_to_balance: Box::new(map::Map::default())
        }
    }

    pub fn name(&self) -> &utf8::Utf8<64> {
        &self.name
    }

    pub fn description(&self) -> &utf8::Utf8<256> {
        &self.description
    }

    pub fn total_supply(&self) -> &q::Q2<u128> {
        &self.total_supply
    }
    
    pub fn transfer(&mut self, from: engine::Address, to: engine::Address, amount: q::Q2<u128>) {

    }

    pub fn mint(&mut self, to: engine::Address, amount: q::Q2<u128>) {
        let balance: q::Q2<u128> = self.address_to_balance
            .get(&to)
            .unwrap_or(&q::as_0())
            .to_owned();
        let balance: q::Q2<u128> = (balance + amount).unwrap();
        self.address_to_balance.insert(to, balance).unwrap();
        self.total_supply = (self.total_supply + amount).unwrap();
    }

    pub fn burn(&mut self, from: engine::Address, amount: q::Q2<u128>) {

    }
}