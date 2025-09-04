use ::dioxus::prelude::*;
use ::gloo_timers::callback as cb;
use ::std::collections as ds;

use ::reliq::array;
use ::reliq::ops;
use ::reliq::map;
use ::reliq::q;
use ::reliq::utf8;

mod engine;
mod game;

static SF_PIXELATE: &str = "SF Pixelate";

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
        link { rel: "stylesheet", href: "https://fonts.cdnfonts.com/css/sf-pixelate" }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    use_future(move || async move {
        game::spawn_logger();
        game::spawn_population(game::PopulationConfiguration {
            celestial_body: engine::Address::new_from_next(),
            max_initial_count: 200000,
            min_initial_count: 100000,
            growth_multiplier: 1_000010.into()
        });
        game::spawn_market(
            game::spawn_item(game::ItemConfiguration {
                name: "Wheat".try_into().unwrap(),
                description: "Make some bread.".try_into().unwrap()
            }),
            game::spawn_item(game::ItemConfiguration {
                name: "Credit".try_into().unwrap(),
                description: "Galactic unit of currency.".try_into().unwrap()
            }),
            5000_00.into(),
            2_00.into()
        );
        
        game::Event::Boot.post();
        cb::Interval::new(1000, {
            move || {
                game::Event::Tick.post();
            }
        }).forget();
    });

    rsx!(
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                min-width: 100vw;
                min-height: 100vh;
                background: #202020;
            "#,

        }
    )
}