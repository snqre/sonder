use super::*;

#[component]
pub fn SpecialButton() -> Element {
    rsx!(
        div {
            style: format!(
                r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    font-size: 2em;
                    font-family: br cobane;
                    font-weight: normal;
                    color: #202020;
                "#
            ),
            ""
        }
    )
}