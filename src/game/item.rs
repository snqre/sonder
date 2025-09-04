use super::*;

pub struct ItemConfiguration {
    pub name: utf8::Utf8<64>,
    pub description: utf8::Utf8<256>
}

pub fn spawn_item(c: ItemConfiguration) -> Address {
    let m_origin: Address = Address::new_from_next();
    let m_name: utf8::Utf8<64> = c.name;
    let m_description: utf8::Utf8<256> = c.description;
    let mut m_total_supply: q::Q2<u128> = q::as_0();
    let mut m_address_to_balance: Box<map::Map<256, Address, q::Q2<u128>>> = Box::new(map::Map::default());

    on(move |event| match event {
        super::Event::Boot => vec!(super::Event::ItemUpdate {
            origin: m_origin.to_owned(),
            source: m_origin.to_owned(),
            name: m_name.to_owned(),
            description: m_description.to_owned(),
            total_supply: m_total_supply.to_owned(),
            address_to_balance: m_address_to_balance.to_owned()
        }),
        super::Event::ItemTransferRequest {
            source,
            sender,
            recipient,
            amount,
            ..
        } => {
            let source: Address = *source;
            let sender: Address = *sender;
            let recipient: Address = *recipient;
            let amount: q::Q2<_> = *amount;
            if source != m_origin {
                return vec!()
            }
            let sender_balance: q::Q2<_> = *m_address_to_balance.get(&sender).unwrap_or(&q::as_0());
            let recipient_balance: q::Q2<_> = *m_address_to_balance.get(&recipient).unwrap_or(&q::as_0());
            let old_sender_balance: q::Q2<_> = sender_balance;
            let new_sender_balance: q::Q2<_> = (old_sender_balance - amount).unwrap();
            let old_recipient_balance: q::Q2<_> = recipient_balance;
            let new_recipient_balance: q::Q2<_> = (old_recipient_balance + amount).unwrap();
            m_address_to_balance.insert(sender, new_sender_balance).unwrap();
            m_address_to_balance.insert(recipient, new_recipient_balance).unwrap();
            vec!(super::Event::ItemUpdate {
                origin: m_origin.to_owned(),
                source: m_origin.to_owned(),
                name: m_name.to_owned(),
                description: m_description.to_owned(),
                total_supply: m_total_supply.to_owned(),
                address_to_balance: m_address_to_balance.to_owned()
            }, super::Event::ItemTransfer {
                origin: m_origin.to_owned(),
                item_name: m_name.to_owned(),
                item_description: m_description.to_owned(),
                sender,
                old_sender_balance,
                new_sender_balance,
                recipient,
                old_recipient_balance,
                new_recipient_balance,
                amount,
                total_supply: m_total_supply.to_owned()
            })
        },
        super::Event::ItemMintRequest {
            source,
            recipient,
            amount,
            ..
        } => {
            if source != &m_origin {
                return vec!()
            }
            let amount: q::Q2<_> = *amount;
            let balance: q::Q2<_> = *m_address_to_balance.get(recipient).unwrap_or(&q::as_0());
            let old_balance: q::Q2<_> = balance;
            let new_balance: q::Q2<_> = balance;
            let new_balance: q::Q2<_> = (new_balance + amount).unwrap();
            m_address_to_balance.insert(*recipient, new_balance).unwrap();
            m_total_supply = (m_total_supply + amount).unwrap();
            vec!(super::Event::ItemMint {
                origin: m_origin,
                source: m_origin,
                recipient: *recipient,
                old_balance,
                new_balance
            })
        },
        super::Event::ItemBurnRequest {
            source,
            sender,
            amount,
            ..
        } => {
            if source != &m_origin {
                return vec!()
            }
            let amount: q::Q2<_> = *amount;
            let balance: q::Q2<_> = *m_address_to_balance.get(sender).unwrap_or(&q::as_0());
            let old_balance: q::Q2<_> = balance;
            let new_balance: q::Q2<_> = balance;
            let new_balance: q::Q2<_> = (new_balance - amount).unwrap();
            m_address_to_balance.insert(*sender, new_balance).unwrap();
            m_total_supply = (m_total_supply - amount).unwrap();
            vec!(super::Event::ItemBurn {
                origin: m_origin,
                source: m_origin,
                sender: *sender,
                old_balance,
                new_balance
            })
        },
        _ => vec!()
    });

    m_origin
}