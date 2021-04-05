// Returning a Vec is difficult. Eventully will be fixed by interface types
// which havent moved much in the past years
// https://github.com/rustwasm/wasm-bindgen/issues/111

// todo is there another faster way to take our struct as input?
use core::ops::Div;
use itertools::iproduct;
use js_sys::Array;
use mimc::{ChunkFootprint, Coords, MimcState, Planet, Response, Task, P, U512};
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub struct Planet3 {
    pub x: i64,
    pub y: i64,
    hash: String,
}

#[wasm_bindgen]
impl Planet3 {
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
// todo this isnt the standard return style we use
#[wasm_bindgen]
pub fn mimc_array(val: &JsValue) -> Array {
    #[allow(non_snake_case)]
    let Task {
        chunkFootprint,
        planetHashKey,
        planetRarity,
    } = val.into_serde().unwrap();

    let x = chunkFootprint.bottomLeft.x;
    let y = chunkFootprint.bottomLeft.y;
    let size = chunkFootprint.sideLength;

    vec![
        Planet3 {
            x: 216,
            y: 158,
            hash: String::from(
                "243586509754089793444036766672578053539607441572992882184488791828676079",
            ),
        },
        Planet3 {
            x: 1100,
            y: 44,
            hash: String::from(
                "2435865097540897934440367666725780535395554992882184488791828676079",
            ),
        },
        Planet3 {
            x: -25,
            y: 33,
            hash: String::from(
                "24358650975408979344403676667257805353960744157299288218448879556676079",
            ),
        },
        Planet3 {
            x: 141441,
            y: 42244,
            hash: String::from(
                "2435456754089793444036766672578053539607441572992882184488791828676079",
            ),
        },
        Planet3 {
            x: 1,
            y: -20000,
            hash: String::from(
                "2435865097540897966666672578053539607441572992882184488791828676079",
            ),
        },
    ]
    .into_iter()
    .map(JsValue::from)
    .collect()
}

// https://github.com/rustwasm/wasm-bindgen/issues/111#issuecomment-527380864
// todo this doesnt send back the chunkfootprint to know which chunk it was
#[wasm_bindgen]
pub fn mimc_array2(val: &JsValue) -> Array {
    #[allow(non_snake_case)]
    let Task {
        chunkFootprint,
        planetHashKey,
        planetRarity,
    } = val.into_serde().unwrap();

    let x = chunkFootprint.bottomLeft.x;
    let y = chunkFootprint.bottomLeft.y;
    let size = chunkFootprint.sideLength;

    vec![
        Planet2 {
            coords: Coords2 { x: 216, y: 158 },
            hash: String::from(
                "243586509754089793444036766672578053539607441572992882184488791828676079",
            ),
        },
        Planet2 {
            coords: Coords2 { x: 1100, y: 44 },
            hash: String::from(
                "2435865097540897934440367666725780535395554992882184488791828676079",
            ),
        },
        Planet2 {
            coords: Coords2 { x: -25, y: 33 },
            hash: String::from(
                "24358650975408979344403676667257805353960744157299288218448879556676079",
            ),
        },
        Planet2 {
            coords: Coords2 {
                x: 141441,
                y: 42244,
            },
            hash: String::from(
                "2435456754089793444036766672578053539607441572992882184488791828676079",
            ),
        },
        Planet2 {
            coords: Coords2 { x: 1, y: -20000 },
            hash: String::from(
                "2435865097540897966666672578053539607441572992882184488791828676079",
            ),
        },
    ]
    .into_iter()
    .map(JsValue::from)
    .collect()
}

// https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
// There are two primary disadvantages. The first is that it is not always
// compatible with the default JSON-based serialization. The second is that it
// performs more calls back and forth between JS and Wasm, which has not been
// fully optimized in all engines, meaning it can sometimes be a speed
// regression.
#[wasm_bindgen]
pub fn mimc_json(val: &JsValue) -> Result<JsValue, JsValue> {
    #[allow(non_snake_case)]
    let Task {
        chunkFootprint,
        planetHashKey,
        planetRarity,
    } = val.into_serde().unwrap();

    let x = chunkFootprint.bottomLeft.x;
    let y = chunkFootprint.bottomLeft.y;
    let size = chunkFootprint.sideLength;

    let planets = vec![
        Planet {
            coords: Coords { x: 216, y: 158 },
            hash: String::from(
                "243586509754089793444036766672578053539607441572992882184488791828676079",
            ),
        },
        Planet {
            coords: Coords { x: 1100, y: 44 },
            hash: String::from(
                "2435865097540897934440367666725780535395554992882184488791828676079",
            ),
        },
        Planet {
            coords: Coords { x: -25, y: 33 },
            hash: String::from(
                "24358650975408979344403676667257805353960744157299288218448879556676079",
            ),
        },
        Planet {
            coords: Coords {
                x: 141441,
                y: 42244,
            },
            hash: String::from(
                "2435456754089793444036766672578053539607441572992882184488791828676079",
            ),
        },
        Planet {
            coords: Coords { x: 1, y: -20000 },
            hash: String::from(
                "2435865097540897966666672578053539607441572992882184488791828676079",
            ),
        },
    ];
    let res = Response {
        planetLocations: planets,
        chunkFootprint,
    };

    serde_wasm_bindgen::to_value(&res).map_err(|err| err.into())
}

// https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen]
pub fn mimc_serde(val: &JsValue) -> JsValue {
    #[allow(non_snake_case)]
    let Task {
        chunkFootprint,
        planetHashKey,
        planetRarity,
    } = val.into_serde().unwrap();

    let x = chunkFootprint.bottomLeft.x;
    let y = chunkFootprint.bottomLeft.y;
    let size = chunkFootprint.sideLength;

    let planets = vec![
        Planet {
            coords: Coords { x: 216, y: 158 },
            hash: String::from(
                "243586509754089793444036766672578053539607441572992882184488791828676079",
            ),
        },
        Planet {
            coords: Coords { x: 1100, y: 44 },
            hash: String::from(
                "2435865097540897934440367666725780535395554992882184488791828676079",
            ),
        },
        Planet {
            coords: Coords { x: -25, y: 33 },
            hash: String::from(
                "24358650975408979344403676667257805353960744157299288218448879556676079",
            ),
        },
        Planet {
            coords: Coords {
                x: 141441,
                y: 42244,
            },
            hash: String::from(
                "2435456754089793444036766672578053539607441572992882184488791828676079",
            ),
        },
        Planet {
            coords: Coords { x: 1, y: -20000 },
            hash: String::from(
                "2435865097540897966666672578053539607441572992882184488791828676079",
            ),
        },
    ];

    let res = Response {
        planetLocations: planets,
        chunkFootprint,
    };
    JsValue::from_serde(&res).unwrap()
}
