use super::*;

pub struct Population {
    count: u128,
    growth_multiplier: f64
}

impl Population {
    pub fn new(count: u128, growth_multiplier: f64) -> Rc<RefCell<Box<Self>>> {
        let ret: Self = Self {
            count,
            growth_multiplier
        };
        let ret: Box<dyn engine::Tick> = Box::new(ret);
        let ret: RefCell<_> = RefCell::new(ret);
        let ret: Rc<_> = Rc::new(ret);
        {
            let ret: Rc<_> = ret.to_owned();
            engine::register(ret);
        }
        unsafe { 
            Rc::from_raw(Rc::into_raw(ret) as *const RefCell<Box<Self>>) 
        }
    }

    pub fn count(&self) -> u128 {
        self.count
    }

    pub fn growth_multiplier(&self) -> f64 {
        self.growth_multiplier
    }
}

impl engine::Tick for Population {
    fn update(&mut self) {
        if self.growth_multiplier == 0.0 {
            return
        }
        let count: f64 = self.count as f64;
        let count: f64 = count * self.growth_multiplier;
        let count: f64 = count.round().max(0.0);
        let count: u128 = count as u128;
        self.count = count;
    }
}