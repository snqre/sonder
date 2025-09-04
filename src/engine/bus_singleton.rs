use super::*;

static BUS: GlobalSignal<Bus<game::Event>> = GlobalSignal::new(|| {
    Bus::default()
});

pub fn post(event: game::Event) {
    BUS.write().post(event);
}

pub fn on<A, B>(mut on_event: A) 
where
    A: FnMut(&game::Event) -> B + 'static,
    B: Into<Option<Vec<game::Event>>> {
    BUS.write().on(move |event| {
        on_event(event).into()
    });
}