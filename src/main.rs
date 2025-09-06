use ::dioxus::prelude::*;
use ::gloo_timers::callback as cb;
use ::std::collections as ds;
use ::std::sync;
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
    use_future(|| async {
        game::connect(game::Logger);
        game::connect(game::Population::new(engine::Address::new_from_next(), 8000000000, 1000000000, 1_000005.into()));
        game::connect(game::Item::new("Credit", ""));
        game::post(game::Event::Boot);
        cb::Interval::new(1000, move || {
            game::post(game::Event::Tick)
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