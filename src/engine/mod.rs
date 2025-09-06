use super::*;

::modwire::expose!(
    pub address
    pub bus
    pub sprite
);

pub trait Rule {
    type Component;

    fn apply(&self, components: &mut Vec::<Self::Component>);
}

pub struct Builder<T> {
    components: Vec<T>,
    rules: Vec<Box<dyn Rule<Component = T>>>
}

impl<A> Builder<A> {
    pub fn add_component<B>(mut self, component: B) -> Self 
    where
        B: Into<A> {
        let component: A = component.into();
        self.components.push(component);
        self
    }

    pub fn add_rule<B>(mut self, rule: B) -> Self 
    where
        B: 'static,
        B: Rule<Component = A> {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn build(self) -> Engine<A> {
        Engine {
            components: self.components,
            rules: self.rules
        }
    }
}

pub struct Engine<T> {
    components: Vec<T>,
    rules: Vec<Box<dyn Rule<Component = T>>>
}

impl<T> Engine<T> {
    pub fn new() -> Builder<T> {
        Builder {
            components: vec!(),
            rules: vec!()
        }
    }

    pub fn update(&mut self) {
        for rule in self.rules.iter_mut() {
            rule.apply(&mut self.components);
        }
    }
}