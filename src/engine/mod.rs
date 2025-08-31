use super::*;

static CHILDREN: GlobalSignal<Vec<Rc<RefCell<Box<dyn Tick>>>>> = GlobalSignal::new(|| {
    vec!()
});

pub trait Tick {
    fn update(&mut self);
}

pub fn register(child: Rc<RefCell<Box<dyn Tick>>>) {
    CHILDREN.write().push(child);
}

// every update cycle represents one day in-game
pub fn update() {
    for child in CHILDREN.write().iter_mut() {
        child.borrow_mut().update();
    }
}