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

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn sponge_vec(x: i32, y: i32, rounds: usize, key: u32) -> Vec<u8> {
    let mut blah = [0u8; 64];

    mimc::MimcState::sponge(vec![x.into(), y.into()], 1, rounds, key)[0]
        .x
        .to_little_endian(&mut blah);

    blah.into()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn sponge_array(x: i32, y: i32, rounds: usize, key: u32) -> Array {
    let mut blah = [0u8; 64];
    mimc::MimcState::sponge(vec![x.into(), y.into()], 1, rounds, key)[0]
        .x
        .to_little_endian(&mut blah);

    blah.iter().map(|x| JsValue::from(*x as u8)).collect()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn sponge_array2(x: i32, y: i32, rounds: usize, key: u32) -> Array {
    let mut blah = [0u8; 64];
    mimc::MimcState::sponge(vec![x.into(), y.into()], 1, rounds, key)[0]
        .x
        .to_little_endian(&mut blah);

    blah.iter().cloned().map(JsValue::from).collect()
}

use js_sys::Uint8Array;
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn sponge_uint8array(x: i32, y: i32, rounds: usize, key: u32) -> Uint8Array {
    let mut blah = [0u8; 64];

    mimc::MimcState::sponge(vec![x.into(), y.into()], 1, rounds, key)[0]
        .x
        .to_little_endian(&mut blah);

    let array: Array = blah.iter().map(|x| JsValue::from(*x as u8)).collect();
    Uint8Array::new(&array)
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn sponge_box(x: i32, y: i32, rounds: usize, key: u32) -> Box<[u8]> {
    let mut blah = [0u8; 64];
    mimc::MimcState::sponge(vec![x.into(), y.into()], 1, rounds, key)[0]
        .x
        .to_little_endian(&mut blah);

    blah.into()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn sponge_string(x: i32, y: i32, rounds: usize, key: u32) -> String {
    mimc::MimcState::sponge(vec![x.into(), y.into()], 1, rounds, key)[0]
        .x
        .to_string()
}
