use super::*;

pub fn spawn_market(
    supply_source: Address, 
    currency_source: Address,
    initial_liquidity: q::Q2<u128>,
    initial_price: q::Q2<u128>) {
    let m_origin: Address = Address::new_from_next();
    let m_supply_source: Address = supply_source;
    let m_currency_source: Address = currency_source;
    let mut m_total_supply: q::Q2<u128> = q::as_0();
    let mut m_total_currency: q::Q2<u128> = q::as_0();
    let mut m_price: q::Q2<u128> = q::as_0();
    let mut m_price_history: array::Array<64, q::Q2<u128>> = array::Array::default();
    let mut m_address_to_balance: Box<map::Map<256, Address, q::Q2<u128>>> = Box::new(map::Map::default());

    on(move |event| {
        match event {
            super::Event::Boot => {
                vec!(
                    super::Event::ItemMintRequest {
                        origin: m_origin,
                        source: m_supply_source,
                        recipient: m_origin,
                        amount: initial_liquidity
                    },
                    super::Event::ItemMintRequest {
                        origin: m_origin,
                        source: m_currency_source,
                        recipient: m_origin,
                        amount: (initial_liquidity * initial_price).unwrap()
                    }
                )
            },
            super::Event::Tick => {
                m_price_history.push(m_price).unwrap();
                vec!(super::Event::MarketUpdate {
                    origin: m_origin,
                    supply_source: m_supply_source,
                    currency_source: m_currency_source,
                    total_supply: m_total_supply,
                    total_currency: m_total_currency,
                    price: m_price,
                    price_history: m_price_history.to_owned()
                })
            },
            super::Event::ItemTransfer {
                origin,
                sender,
                recipient,
                new_sender_balance,
                new_recipient_balance,
                amount,
                ..
            } => {
                m_address_to_balance.insert(*sender, *new_sender_balance);
                m_address_to_balance.insert(*recipient, *new_recipient_balance);
                if origin == &m_supply_source {
                    m_total_supply = (m_total_supply + *amount).unwrap();
                }
                if origin == &m_currency_source {
                    m_total_currency = (m_total_currency + *amount).unwrap();
                }
                if m_total_supply > 0.into() {
                    m_price = (m_total_currency / m_total_supply).unwrap();
                }
                vec!(super::Event::MarketUpdate {
                    origin: m_origin,
                    supply_source: m_supply_source,
                    currency_source: m_currency_source,
                    total_supply: m_total_supply,
                    total_currency: m_total_currency,
                    price: m_price,
                    price_history: m_price_history.to_owned()
                })
            },
            _ => vec!()
        }
    });
}