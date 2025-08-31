use super::*;

#[component]
pub fn Resource(
    count: Signal<f64>,
    icon_url: Asset
) -> Element {
    rsx!(
        div {
            style: format!(
                r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                "#
            ),
            div {
                style: format!(
                    r#"
                        background-image: url({});
                        background-size: contain;
                        background-position-x: center;
                        background-position-y: center;
                        background-repeat: no-repeat;
                    "#,
                    icon_url
                )
            }
            div {
                style: format!(
                    r#"
                        display: flex;
                        flex-direction: row;
                        justify-content: center;
                        align-items: center;
                        font-size: 1em;
                        font-family: br cobane;
                        font-weight: normal;
                    "#,
                    { format!("{}", count) }
                )
            }
        }
    )
}