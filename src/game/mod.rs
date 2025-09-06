use super::GlobalSignal;
use super::Writable as _;
use super::Readable as _;
use super::engine;
use super::q;
use super::utf8;
use super::map;
use super::array;

::modwire::expose!(
    pub item
    pub logger
    pub market
    pub population
);

pub static BUS: GlobalSignal<engine::Bus<Event>> = GlobalSignal::new(|| {
    engine::Bus::new()
});

pub type OnCompletion = Box<dyn Fn(Result<()>)>;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Error {
    InsufficientBalance
}

pub struct ItemTransferRequest {
    pub on_completion: OnCompletion,
    pub addr: engine::Address,
    pub item: engine::Address,
    pub from: engine::Address,
    pub to: engine::Address,
    pub amount: q::Q2<u128>
}

pub struct ItemTransfer {
    addr: engine::Address,
    item: engine::Address,
    from: engine::Address,
    to: engine::Address,
    amount: q::Q2<u128>
}

pub struct ItemMintRequest {
    on_completion: OnCompletion,
    addr: engine::Address,
    item: engine::Address,
    to: engine::Address,
    amount: q::Q2<u128>
}

pub struct ItemMint {
    addr: engine::Address,
    item: engine::Address,
    to: engine::Address,
    amount: q::Q2<u128>
}

pub struct ItemBurnRequest {
    on_completion: OnCompletion,
    addr: engine::Address,
    item: engine::Address,
    from: engine::Address,
    amount: q::Q2<u128>
}

pub struct ItemBurn {
    addr: engine::Address,
    item: engine::Address,
    from: engine::Address,
    amount: q::Q2<u128>
}

pub enum Event {
    Boot,
    Tick,
    ItemSpawn(Item),
    ItemTransferRequest(ItemTransferRequest),
    ItemTransfer(Item, ItemTransfer),
    ItemMintRequest(ItemMintRequest),
    ItemMint(Item, ItemMint),
    ItemBurnRequest(ItemBurnRequest),
    ItemBurn(Item, ItemBurn),






    PopulationSpawn(Population),
    PopulationUpdate(Population),

    
    MarketSpawn(Market),
    MarketUpdate(Market),
    MarketPurchaseRequest {
        on_completion: OnCompletion,
        addr: engine::Address,
        from: engine::Address,
        amount: q::Q2<u128>
    },
    
    MarketSellOrder {
        
    }
}

pub fn post(event: Event) {
    BUS.write().post(event);
}

pub fn connect<T>(service: T)
where
    T: engine::Service<Event = Event> + 'static {
    BUS.write().connect(service);
}