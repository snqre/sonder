use super::*;

#[component]
pub fn Chart(
    w: f64,
    h: f64,
    market: Signal<Market>
) -> Element {
    let price_history: Vec<(f64, f64)> = market
        .read()
        .price_history()
        .iter()
        .enumerate()
        .map(|(k, p)| (k as f64, *p))
        .collect();

    rsx!(
        div {
            style: r#"
                min-width: {w}px;
                min-height: {h}px;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                border-style: solid;
                border-color: #202020;
                border-width: 1px;
                border-radius: 4px;
                padding: 8px;
            "#,
            svg {
                width: w,
                height: h,
                polyline {
                    style: r#"
                        fill: none;
                        stroke: #ffffff;
                        stroke-width: 2px;
                    "#,
                    points: serialize_points(&price_history, w, h),
                }
                for p in p_labels(&price_history, 8) {{
                    let (
                        _,
                        _,
                        min_p,
                        _,
                        _,
                        p_range
                    ) = parse_points(&price_history);
                    let y: f64 = h - ((p - min_p) / p_range) * h;
                    rsx!(
                        text {
                            style: r#"
                                fill: black;
                                font-size: 1em;
                                font-family: br cobane;
                                font-weight: normal;
                                text-anchor: end;
                            "#,
                            x: w,
                            y,
                            "£{p:.2}"
                        }
                    )
                }}
            }
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    font-size: 1em;
                    font-family: {SF_PIXELATE};
                    font-weight: normal;
                "#,
                {{
                    let player_uuid: _ = uuid::PLAYER.read();
                    let player_balance: f64 = market.read().balance_of(&player_uuid);
                    let player_balance: String = format!("{:.2}", player_balance);
                    rsx!(
                        div {
                            { player_balance }
                        }
                    )
                }}
            }
        }
    )
}

fn p_labels(points: &Vec<(f64, f64)>, segments: usize) -> Vec<f64> {
    let (
        _, 
        _, 
        min_p, 
        _, 
        _, 
        p_range
    ) = parse_points(points);
    (0..=segments)
        .map(|k| min_p + k as f64 * (p_range / segments as f64))
        .collect()
}

fn serialize_points(points: &Vec<(f64, f64)>, w: f64, h: f64) -> String {
    let mut ret: String = String::new();
    let (
        min_t,
        _,
        min_p,
        _,
        t_range,
        p_range
    ) = parse_points(points);
    for point in points {
        let t: f64 = point.0;
        let p: f64 = point.1;
        let x: f64 = ((t - min_t) / t_range) * w;
        let y: f64 = h - ((p - min_p) / p_range) * h;
        ret.push_str(&format!("{},{}", x, y));
        ret.push(' ');
    }
    ret
}

fn parse_points(points: &Vec<(f64, f64)>) -> (
    f64, 
    f64, 
    f64, 
    f64, 
    f64, 
    f64) {
    let mut min_t: f64 = f64::INFINITY;
    let mut max_t: f64 = f64::NEG_INFINITY;
    let mut min_p: f64 = f64::INFINITY;
    let mut max_p: f64 = f64::NEG_INFINITY;
    for point in points {
        let t: f64 = point.0;
        let p: f64 = point.1;
        if t < min_t {
            min_t = t;
        }
        if t > max_t {
            max_t = t;
        }
        if p < min_p {
            min_p = p;
        }
        if p > max_p {
            max_p = p;
        }
    }
    (
        min_t,
        max_t,
        min_p,
        max_p,
        max_t - min_t,
        max_p - min_p
    )
}