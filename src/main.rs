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
    use_effect(|| {
        let mut day: ::game::Day = 1;
        let (_, asteroid) = ::game::Asteroid::new();
        ::game::lock(|b| {
            b.connect(::game::Logger);
            b.connect(asteroid);
            b.post(::game::Event::Boot);
        });
        cb::Interval::new(1000, move || {
            ::game::lock(|b| {
                b.post(::game::Event::DayTermination(day));
            });
            day += 1;
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
            "#
        }
    )
}