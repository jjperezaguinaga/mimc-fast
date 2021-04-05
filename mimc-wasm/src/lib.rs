// Returning a Vec is difficult. Eventully will be fixed by interface types
// which havent moved much in the past years
// https://github.com/rustwasm/wasm-bindgen/issues/111

// todo is there another faster way to take our struct as input?
use core::ops::Div;
use itertools::iproduct;
use js_sys::Array;
use mimc::{MimcState, Task, P, U512};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// we have to redefine the structs so they have #[wasm_bindgen] applied also I
// wasnt seeing coords struct come through even then, so just flatten x and y
// into planet
#[wasm_bindgen]
pub struct Planet2 {
    pub x: i64,
    pub y: i64,
    hash: String,
}

// also you cant directly pass strings without this
#[wasm_bindgen]
impl Planet2 {
    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }
}

// https://github.com/rustwasm/wasm-bindgen/issues/111#issuecomment-527380864
#[wasm_bindgen]
pub fn mimc(val: &JsValue) -> Array {
    #[allow(non_snake_case)]
    let Task {
        chunkFootprint,
        planetHashKey,
        planetRarity,
    } = val.into_serde().unwrap();

    let x = chunkFootprint.bottomLeft.x;
    let y = chunkFootprint.bottomLeft.y;
    let size = chunkFootprint.sideLength;

    iproduct!(x..(x + size), y..(y + size))
        .into_iter()
        .filter_map(|(xi, yi)| {
            let hash = MimcState::sponge(vec![xi, yi], 1, 220, planetHashKey)[0].x;
            if hash < P.div(U512::from(planetRarity)) {
                Some(Planet2 {
                    x: xi,
                    y: yi,
                    hash: hash.to_string(),
                })
            } else {
                None
            }
        })
        .map(JsValue::from)
        .collect()
}
