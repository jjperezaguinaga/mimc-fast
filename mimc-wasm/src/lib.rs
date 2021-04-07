#[wasm_bindgen::prelude::wasm_bindgen]
pub fn sponge(x: i32, y: i32, rounds: usize, key: u32) -> String {
    mimc::MimcState::sponge(&[x.into(), y.into()], rounds, key).to_string()
}
