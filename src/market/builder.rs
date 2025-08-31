use super::*;

#[derive(Debug)]
#[derive(Default)]
pub struct Builder {
    uuid: Option<uuid::Uuid>,
    name: Option<String>,
    description: Option<String>,
    symbol: Option<String>,
    credit: Option<f64>,
    supply: Option<f64>,
    uuid_to_balance: ds::HashMap<uuid::Uuid, f64>
}

impl Builder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn with_initial_price(mut self, price: f64) -> Self {
        self.supply = Some(1.0);
        self.credit = Some(price);
        self
    }

    pub fn with_liquidity(mut self, additional_supply: f64) -> Self {
        match (self.credit, self.supply) {
            (Some(credit), Some(supply)) => {
                let price: f64 = credit / supply;
                let additional_credit: f64 = additional_supply * price;
                self.credit = Some(credit + additional_credit);
                self.supply = Some(supply + additional_supply);
            },
            _ => {
                self.credit = Some(additional_supply);
                self.supply = Some(additional_supply);
            }
        }
        self
    }

    pub fn build(self) -> Option<Market> {
        Some(Market {
            uuid: self.uuid.unwrap_or_default(),
            name: self.name?,
            description: self.description?,
            symbol: self.symbol?,
            total_credit: self.credit?,
            total_supply: self.supply?,
            uuid_to_balance: self.uuid_to_balance
        })
    }
}