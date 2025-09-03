use super::*;

pub fn spawn_logger() {
    use ::web_sys::wasm_bindgen;
    use ::web_sys::console;
    on(|event| {
        let event: &wasm_bindgen::JsValue = &format!("{:?}", event).into();
        console::log_1(event);
        vec![]
    });
}