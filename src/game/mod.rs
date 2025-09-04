use super::*;
use super::engine::*;

::modwire::expose!(
//    pub celestial_body
//    pub galaxy
    pub logger
    pub market
    pub item
    pub population
);

#[derive(Debug)]
#[derive(Clone)]
pub enum Event {
    /// an event that is only fired at the start of the program.
    Boot,
    Tick,


    PopulationUpdate {
        origin: Address,
        celestial_body: Address,
        min_initial_count: u128,
        max_initial_count: u128,
        growth_multiplier: q::Q6<u128>,
        old_count: u128,
        new_count: u128
    },





    ItemUpdate {
        origin: Address,
        source: Address,
        name: utf8::Utf8<64>,
        description: utf8::Utf8<256>,
        total_supply: q::Q2<u128>,
        address_to_balance: Box<map::Map<256, Address, q::Q2<u128>>>
    },
    ItemTransferRequest {
        origin: Address,
        source: Address,
        sender: Address,
        recipient: Address,
        amount: q::Q2<u128>
    },
    ItemTransfer {
        origin: Address,
        item_name: utf8::Utf8<64>,
        item_description: utf8::Utf8<256>,
        sender: Address,
        old_sender_balance: q::Q2<u128>,
        new_sender_balance: q::Q2<u128>,
        recipient: Address,
        old_recipient_balance: q::Q2<u128>,
        new_recipient_balance: q::Q2<u128>,
        amount: q::Q2<u128>,
        total_supply: q::Q2<u128>
    },
    ItemMintRequest {
        origin: Address,
        source: Address,
        recipient: Address,
        amount: q::Q2<u128>
    },
    ItemMint {
        origin: Address,
        source: Address,
        recipient: Address,
        old_balance: q::Q2<u128>,
        new_balance: q::Q2<u128>
    },
    ItemBurnRequest {
        origin: Address,
        source: Address,
        sender: Address,
        amount: q::Q2<u128>
    },
    ItemBurn {
        origin: Address,
        source: Address,
        sender: Address,
        old_balance: q::Q2<u128>,
        new_balance: q::Q2<u128>
    },






    InsufficientItemBalance {
        origin: Address,
        
    },
    MarketUpdate {
        origin: Address,
        supply_source: Address,
        currency_source: Address,
        total_supply: q::Q2<u128>,
        total_currency: q::Q2<u128>,
        price: q::Q2<u128>,
        price_history: array::Array<64, q::Q2<u128>>
    }
}