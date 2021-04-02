use core::ops::Div;
use itertools::iproduct;
use mimc::{ChunkFootprint, Coords, MimcState, Planet, P, U512};
use wasm_bindgen::prelude::*;

use js_sys::Array;

// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Coords2 {
    pub x: i64,
    pub y: i64,
}

#[wasm_bindgen]
pub struct Planet2 {
    coords: Coords2,
    hash: String,
}

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

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn mimc() -> Array {
    let chunk_footprint = ChunkFootprint {
        bottomLeft: Coords { x: 0, y: 0 },
        sideLength: 16,
    };

    let planet_hash_key = 8;
    let planet_rarity = 16384;
    let x = chunk_footprint.bottomLeft.x;
    let y = chunk_footprint.bottomLeft.y;
    let size = chunk_footprint.sideLength;

    let threshold = P.div(U512::from(planet_rarity));

    iproduct!(x..(x + size), y..(y + size))
        .into_iter()
        .filter_map(|(xi, yi)| {
            let hash = MimcState::sponge(vec![xi, yi], 1, 220, planet_hash_key)[0].x;
            if hash < threshold {
                Some(Planet2 {
                    coords: Coords2 { x: xi, y: yi },
                    hash: hash.to_string(),
                })
            } else {
                None
            }
        })
        .map(JsValue::from)
        .collect()
}
