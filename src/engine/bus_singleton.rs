use super::*;

static BUS: GlobalSignal<Bus<game::Event>> = GlobalSignal::new(|| {
    Bus::default()
});

pub fn post(event: game::Event) {
    BUS.write().post(event);
}

pub fn on<T>(on_event: T) 
where
    T: FnMut(&game::Event) -> Vec<game::Event> + 'static {
    BUS.write().on(on_event);
}