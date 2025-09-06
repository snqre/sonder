use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct Market {
    pub addr: engine::Address,
    pub supply: engine::Address,
    pub supply_snapshot: Option<Item>,
    pub assets: engine::Address,
    pub assets_snapshot: Option<Item>,
    pub initial_liquidity: q::Q2<u128>,
    pub initial_price: q::Q2<u128>,
    pub total_supply: q::Q2<u128>,
    pub total_assets: q::Q2<u128>,
    pub price_history: array::Array<64, q::Q2<u128>>
}

impl Market {
    pub fn price(&self) -> Option<q::Q2<u128>> {
        (self.total_assets / self.total_supply).ok()
    }

    pub fn price_history_is_full(&self) -> bool {
        self.price_history.len() >= self.price_history.cap()
    }

    // pre-check
    pub fn check_sale(&self) {
        self.supply_snapshot
            .to_owned()
            .unwrap()
            .will_transfer();
        self.assets_snapshot
            .to_owned()
            .unwrap()
            .will_transfer();
    }
}

impl engine::Service for Market {
    type Event = Component;

    fn receive(&mut self, event: &Self::Event) -> Option<Vec<Self::Game>> {
        match event {
            Component::Boot => Some(vec!(
                Component::ItemMintRequest(ItemMintRequest({
                    addr: self.addr,
                    item: self.supply,
                    to: self.addr,
                    amount: self.initial_liquidity,
                    on_completion: Box::new(move |outcome| outcome.unwrap())
                })),
                Event::ItemMintRequest {
                    addr: self.addr,
                    item: self.assets,
                    to: self.addr,
                    amount: (self.initial_liquidity * self.initial_price).unwrap(),
                    on_completion: Box::new(move |outcome| outcome.unwrap())
                }
            )),
            Component::Tick => {
                if self.price_history_is_full() {
                    self.price_history.remove(0).unwrap();
                }
                Some(vec!(Component::MarketUpdate(self.to_owned())))
            },
            Component::ItemSpawn(item) => {
                if item.addr == self.supply {
                    self.supply_snapshot = Some(item.to_owned())
                }
                if item.addr == self.assets {
                    self.assets_snapshot = Some(item.to_owned())
                }
                None
            },
            Component::ItemTransfer(ItemTransfer {
                addr,
                item,
                sender,
                recipient,
                old_sender_balance,
                old_recipient_balance,
                new_sender_balance,
                new_recipient_balance
            }) => {
                if recipient != &self.addr || (item != &self.supply || item != &self.assets) {
                    return None
                }
                if item == &self.supply {
                
                }

                Some(vec!(Component::ItemTransferRequest {
                    addr,
                    item,
                    from,
                    to,
                    amount,
                    on_completion: Box::new(|_| {
                        
                    })
                }))
            },




            Component::MarketPurchaseRequest(request) => {
                if request.market != self.addr {
                    return None
                }
                let Some(price) = self.price() else {
                    return None
                };
                // post()
                (request.amount * self.price().unwrap()).unwrap();
                Some(vec!(Component::ItemTransferRequest(ItemTransferRequest {
                    addr: self.addr,
                    item: self.supply,
                    from: self.addr,
                    to,
                    amount
                })))
            },

            Component::MarketPurchaseRequestStage2 => {

            }


            Component::MarketSaleRequest(request) => {
                if request.market != self.addr {
                    return None
                }

                // 

                let Some(price) = self.price() else {
                    return Some(vec!(Component::MarketSaleRequestFailure()))
                };
                let Some(supply_snapshot) = self.supply_snapshot else {
                    return Some(vec!(Component::MarketSaleRequestFailure()))
                };
                let Some(assets_snapshot) = self.assets_snapshot else {
                    return Some(vec!(Component::MarketSaleRequestFailure()))
                };
                let supply_balance: &q::Q2<_> = supply_snapshot.address_to_balance.get(&request.from).unwrap_or(&q::as_0());
                if supply_balance < &request.amount {
                    return Some(vec!(Component::MarketSaleRequestFailure()))
                }
                let assets_out: q::Q2<_> = (request.amount * price).unwrap();
                let market_assets_balance: &q::Q2<_> = assets_snapshot.address_to_balance.get(&self.addr).unwrap_or(&q::as_0());
                if market_assets_balance < &assets_out {
                    panic!("...")
                }
                self.total_supply = (self.total_supply + request.amount).unwrap();
                self.total_assets = (self.total_assets - assets_out).unwrap();
                Some(vec!(
                    Component::ItemTransferRequest(ItemTransferRequest {
                        addr: self.addr,
                        item: self.supply,
                        from: request.from,
                        to: self.addr,
                        amount: request.amount
                    }),
                    Component::ItemTransferRequest(ItemTransferRequest {
                        addr: self.addr,
                        item: self.assets,
                        from: self.addr,
                        to: request.from,
                        amount: assets_out
                    }),
                    Component::MarketSale(self.to_owned()),
                    Component::MarketUpdate(self.to_owned())
                ))
            },
            _ => None
        }
    }
}