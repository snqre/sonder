use super::*;

pub fn spawn_source(
    name: utf8::Utf8<64>,
    description: utf8::Utf8<256>) {
    let m_origin: Address = Address::new_from_next();
    let m_name: utf8::Utf8<64> = name;
    let m_description: utf8::Utf8<256> = description;
    let mut m_total_supply: q::Q2<u128> = q::as_0();
    let mut m_address_to_balance: Box<map::Map<256, Address, q::Q2<u128>>> = Box::new(map::Map::default());

    on(move |event| {
        match event {
            super::Event::Boot => vec!(super::Event::ItemSpawn {
                origin: m_origin,
                name: m_name.to_owned(),
                description: m_description.to_owned()
            }),
            super::Event::ItemTransferRequest {
                origin,
                sender,
                recipient,
                amount
            } => {
                let curr_sender_balance: q::Q2<u128> = *m_address_to_balance.get(sender).unwrap_or(&q::as_0());
                let curr_recipient_balance: q::Q2<u128> = *m_address_to_balance.get(recipient).unwrap_or(&q::as_0());
                if curr_sender_balance < *amount {
                    return vec!()
                }
                let Ok(new_sender_balance) = curr_sender_balance - *amount else {
                    return vec!()
                };
                let Ok(new_recipient_balance) = curr_recipient_balance + *amount else {
                    return vec!()
                };
                m_address_to_balance.insert(*sender, new_sender_balance);
                m_address_to_balance.insert(*recipient, new_recipient_balance);
                vec!(super::Event::ItemTransfer {
                    origin: m_origin,
                    item_name: m_name.to_owned(),
                    item_description: m_description.to_owned(),
                    sender: sender.to_owned(),
                    old_sender_balance: curr_sender_balance,
                    new_sender_balance,
                    recipient: recipient.to_owned(),
                    old_recipient_balance: curr_recipient_balance,
                    new_recipient_balance,
                    amount: amount.to_owned(),
                    total_supply: m_total_supply.to_owned()
                })
            },
            _ => vec!()
        }
    });
}