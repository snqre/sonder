use super::*;

::modwire::expose!(
    pub builder
);

pub struct Market {
    uuid: uuid::Uuid,
    name: String,
    description: String,
    symbol: String,
    total_credit: f64,
    total_supply: f64,
    uuid_to_balance: ds::HashMap<uuid::Uuid, f64>
}

impl Market {
    pub fn new() -> Builder {
        Builder {
            ..Default::default()
        }
    }

    pub fn balance_of(&self, wallet: &uuid::Uuid) -> f64 {
        *self.uuid_to_balance.get(wallet).unwrap_or(&0.0)
    }

    pub fn price(&self) -> f64 {
        if self.total_supply == 0.0 {
            0.0
        } else {
            self.total_credit / self.total_supply
        }
    }

    pub fn transfer_from(&mut self, sender: &uuid::Uuid, recipient: &uuid::Uuid, supply: f64) -> Option<()> {
        self.normalize_balance(sender);
        self.normalize_balance(recipient);
        if self.balance_of(&sender) < supply {
            return None
        }
        *self.uuid_to_balance.get_mut(&sender).unwrap() -= supply;
        *self.uuid_to_balance.get_mut(&recipient).unwrap() += supply;
        Some(())
    }

    pub fn add_liquidity(&mut self, credit: f64) {
        if self.total_supply == 0.0 {
            self.total_credit = credit;
            self.total_supply = credit;
        } else {
            let price: f64 = self.price();
            let minted_supply: f64 = credit / price;
            self.total_credit = self.total_credit + credit;
            self.total_supply = self.total_supply + minted_supply;
        }
    }

    pub fn remove_liquidity(&mut self, credit: f64) -> Option<()> {
        if credit > self.total_supply {
            return None
        }
        let price: f64 = self.price();
        let credit_removed: f64 = credit * price;
        self.total_credit = self.total_credit - credit_removed;
        self.total_supply = self.total_supply - credit;
        Some(())
    }

    pub fn mint(&mut self, to: &uuid::Uuid, supply: f64) -> Option<()> {
        self.total_supply += supply;
        let Some(balance) = self.uuid_to_balance.get_mut(to) else {
            return None
        };
        *balance += supply;
        Some(())
    }

    pub fn burn(&mut self, from: &uuid::Uuid, supply: f64) -> Option<()> {
        let balance: &mut f64 = self.uuid_to_balance.get_mut(from)?;
        if *balance < supply {
            return None
        }
        *balance -= supply;
        self.total_supply -= supply;
        Some(())
    }

    fn normalize_balance(&mut self, trader: &uuid::Uuid) {
        if self.uuid_to_balance.get(trader).is_none() {
            let trader = trader.to_owned();
            self.uuid_to_balance.insert(trader, 0.0);
        }
    }
}