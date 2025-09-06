use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct Item {
    pub addr: engine::Address,
    pub name: utf8::Utf8<64>,
    pub description: utf8::Utf8<256>,
    pub total_supply: q::Q2<u128>,
    pub address_to_balance: Box<map::Map<256, engine::Address, q::Q2<u128>>>
}

impl Item {
    pub fn new<A, B>(name: A, description: B) -> Self 
    where
        A: TryInto<utf8::Utf8<64>>,
        B: TryInto<utf8::Utf8<256>> {
        Self {
            addr: engine::Address::new_from_next(),
            name: name
                .try_into()
                .ok()
                .unwrap(),
            description: description
                .try_into()
                .ok()
                .unwrap(),
            total_supply: q::as_0(),
            address_to_balance: Box::new(map::Map::new())
        }
    }
}

impl engine::Service for Item {
    type Event = Event;

    fn receive(&mut self, event: &Self::Event) -> Option<Vec<Self::Event>> {
        match event {
            Event::Boot => Some(vec!(Event::ItemSpawn(self.to_owned()))),
            Event::ItemTransferRequest(request) => {
                if request.item != self.addr {
                    return None
                }
                let sender_balance: q::Q2<u128> = self.address_to_balance
                    .get(&request.from)
                    .unwrap_or(&q::as_0())
                    .to_owned();
                let recipient_balance: q::Q2<u128> = self.address_to_balance
                    .get(&request.to)
                    .unwrap_or(&q::as_0())
                    .to_owned();
                if sender_balance < request.amount {
                    (request.on_completion)(Err(Error::InsufficientBalance));
                    return None
                }
                let sender_balance: q::Q2<u128> = (sender_balance - request.amount).unwrap();
                let recipient_balance: q::Q2<u128> = (recipient_balance + request.amount).unwrap();
                self.address_to_balance.insert(request.from, sender_balance).unwrap();
                self.address_to_balance.insert(request.to, recipient_balance).unwrap();
                (request.on_completion)(Ok(()));
                Some(vec!(Event::ItemTransfer(self.to_owned(), ItemTransfer {
                    addr: self.addr,
                    item: self.addr,
                    from: request.from,
                    to: request.to,
                    amount: request.amount
                })))
            },
            Event::ItemMintRequest(request) => {

                None
            },
            _ => None
        }
    }
}