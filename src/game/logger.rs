use super::*;

#[repr(transparent)]
pub struct Logger;

impl engine::Service for Logger {
    type Event = Component;

    fn receive(&mut self, event: &Self::Event) -> Option<Vec<Self::Event>> {
        use ::web_sys::wasm_bindgen;
        use ::web_sys::console;
        let event: &wasm_bindgen::JsValue = &format!("{:?}", event).into();
        console::log_1(event);
        None
    }
}