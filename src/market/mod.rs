use super::*;

::modwire::expose!(
    pub builder
    pub chart
    pub error
);

pub struct Market {
    uuid: uuid::Uuid,
    name: String,
    description: String,
    symbol: String,
    total_credit: q::Q2<u128>,
    total_supply: q::Q2<u128>,
    uuid_to_balance: ds::HashMap<uuid::Uuid, q::Q2<u128>>,
    price_history: Vec<q::Q2<u128>>
}

impl Market {
    pub fn new() -> Builder {
        Builder::new()
    }

    pub fn balance_of(&self, uuid: &uuid::Uuid) -> q::Q2<u128> {
        *self.uuid_to_balance.get(uuid).unwrap_or(&q::as_0())
    }

    pub fn total_credit(&self) -> q::Q2<u128> {
        self.total_credit
    }

    pub fn total_supply(&self) -> q::Q2<u128> {
        self.total_supply
    }

    pub fn market_cap(&self) -> Result<q::Q2<u128>> {
        let x: q::Q2<u128> = self.total_supply();
        let y: q::Q2<u128> = self.price()?;
        Ok((x * y)?)
    }

    pub fn price(&self) -> Result<q::Q2<u128>> {
        if self.total_supply == 0.into() {
            Ok(q::as_0())
        } else {
            Ok((self.total_credit / self.total_supply)?)
        }
    }

    pub fn price_history(&self) -> &Vec<q::Q2<u128>> {
        &self.price_history
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

    pub fn add_liquidity(&mut self, credit: q::Q2<u128>) -> Result<()> {
        if self.total_supply == 0.into() {
            self.total_credit = credit;
            self.total_supply = credit;
            return Ok(())
        }
        let price: q::Q2<u128> = self.price()?;
        let minted_supply: q::Q2<u128> = (credit / price)?;
        self.total_credit = (self.total_credit + credit)?;
        self.total_supply = (self.total_supply + minted_supply)?;
        Ok(())
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
        self.normalize_balance(to);
        self.total_supply += supply;
        let Some(balance) = self.uuid_to_balance.get_mut(to) else {
            return None
        };
        *balance += supply;
        Some(())
    }

    pub fn burn(&mut self, from: &uuid::Uuid, supply: f64) -> Option<()> {
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

impl common::Update for Market {
    fn update(&mut self) {
        let price: f64 = self.price();
        self.price_history.push(price);
    }
}