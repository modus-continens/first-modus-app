use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn greet() {
  let _ = window().unwrap().alert_with_message("Hello from Rust!");
}
