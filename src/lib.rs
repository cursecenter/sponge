use wasm_bindgen::prelude::*;

mod sponge;

#[wasm_bindgen]
pub fn generate_sponge() -> String {
    sponge::generate()
}
