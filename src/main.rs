use ::dioxus::prelude::*;
use ::gloo_timers::callback as cb;
use ::std::collections as ds;
use ::std::rc::Rc;
use ::std::cell::RefCell;

use engine::Tick as _;
use component::*;

mod common;
mod component;
mod engine;
mod galaxy;
mod location;
mod market;
mod name;
mod population;
mod uuid;

type Rbox<T> = Rc<RefCell<Box<T>>>;

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
                    engine::update();
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