use super::*;

static COUNT: GlobalSignal<u128> = Signal::global(|| 0);

pub fn next() -> u128 {
    *COUNT.write() += 1;
    COUNT()
}