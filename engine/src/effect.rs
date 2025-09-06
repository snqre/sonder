use super::*;

pub enum Effect<T> {
    None,
    Kill,
    Emit(Vec<T>),
    Defer(T),
    Spawn(Box<dyn Service<Event = T>>),
    Batch(Vec<Effect<T>>)
}

impl<T> Effect<T> {
    pub fn flatten(self) -> Vec<T> {
        match self {
            Self::None => vec!(),
            Self::Kill => vec!(),
            Self::Emit(events) => events,
            Self::Defer(event) => vec!(event),
            Self::Spawn(_) => vec!(),
            Self::Batch(effects) => effects
                .into_iter()
                .flat_map(|effect| {
                    effect.flatten()
                })
                .collect()
        }
    }
}