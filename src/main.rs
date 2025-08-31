use ::dioxus::prelude::*;
use ::gloo_timers::callback as cb;
use ::std::collections as ds;

mod common;
mod component;
mod galaxy;
mod game;
mod location;
mod market;
mod name;
mod uuid;





#[derive(Debug)]
#[derive(Clone)]
#[derive(Routable)]
#[derive(PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {}
}

fn main() {
    dioxus::launch(Main);
}

#[component]
fn Main() -> Element {
    rsx! {
        document::Stylesheet { href: "/asset/main.css" }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let points: Signal<Vec<(f64, f64)>> = use_signal(|| vec!(
        (1.0, 2.0),
        (2.0, 29.0),
        (3.0, 42.0),
        (4.0, 18.0),
        (5.0, 29.0),
        (6.0, 48.9),
        (7.0, 182.9),
        (8.0, 200.0),
        (9.0, 190.0)
    ));

    use_effect({
        let points: Signal<_> = points.to_owned();
        move || {
            let points: Signal<_> = points.to_owned();
            let _ = cb::Interval::new(1000, {
                let mut points: Signal<_> = points.to_owned();
                move || {
                    let mut new_points: Vec<(f64, f64)> = points();
                    

                    let last = new_points.last().unwrap();
                    let new_t = last.0 + 1.0;
                    let new_p = last.1 * 1.1;
                    new_points.push((new_t, new_p));
                    s.set(new_points);
                }
            });
        }
    });

    rsx!(
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                background: #FFFFFF;
            "#,
            Chart {
                w: 500.0,
                h: 200.0,
                points: points()
            }
        }
    )
}




// resources
// - credit
// - 




pub struct Game {
    pool: Pool
}


pub struct PlayerCompany {
    balance: f64
}


pub struct Planet {

}


pub struct Pool {
    assets: f64,
    supply: f64,
}

impl Pool {
    pub fn new(initial_price: f64) -> Self {

    }

    pub fn buy(&self) {

    }

    pub fn add_liquidity(&mut self, supply_in: f64) {
        self.assets = ;
        self.supply = self.supply + ;
    }

    pub fn remove_liquidity(&mut self, supply_in: f64) {
        
    }

    pub fn price(&self) -> f64 {
        self.assets / self.supply
    }
}




pub trait Trader {

}

#[derive(Debug)]
#[derive(Clone)]
pub struct Investor {
    pub income: f64,
    pub balance: f64,
}

impl Investor {
    pub fn sentiment(&self) {
        ::fastrand::f64();
    }
}