use http_types::headers::HeaderValue;
use itertools::iproduct;
use mimc::{MimcState, P, U512};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Div;
use tide::security::{CorsMiddleware, Origin};
use tide::{Body, Request};
use tide_acme::{AcmeConfig, TideRustlsExt};

#[derive(Serialize, Deserialize, Clone)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Planet {
    pub coords: Coords,
    pub hash: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct ChunkFootprint {
    pub bottomLeft: Coords,
    pub sideLength: i64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub chunkFootprint: ChunkFootprint,
    pub planetRarity: u32,
    pub planetHashKey: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    pub chunkFootprint: ChunkFootprint,
    pub planetLocations: Vec<Planet>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let path = std::env::var("TIDE_ACME_CACHE_DIR").unwrap_or_else(|_| String::from("cache"));

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    let mut app = tide::new();
    app.with(cors);

    app.at("/mine").post(|mut req: Request<()>| async move {
        #[allow(non_snake_case)]
        let Task {
            chunkFootprint,
            planetHashKey,
            planetRarity,
        } = req.body_json().await?;

        let x = chunkFootprint.bottomLeft.x;
        let y = chunkFootprint.bottomLeft.y;
        let size = chunkFootprint.sideLength;
        let key = planetHashKey;

        let threshold = P.div(U512::from(planetRarity));

        let planets = iproduct!(x..(x + size), y..(y + size))
            .par_bridge()
            .filter_map(|(xi, yi)| {
                let hash = MimcState::sponge(&[xi, yi], 220, key);
                if hash < threshold {
                    Some(Planet {
                        coords: Coords { x: xi, y: yi },
                        hash: hash.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<Planet>>();

        let rsp = Response {
            chunkFootprint,
            planetLocations: planets,
        };

        Ok(Body::from_json(&rsp)?)
    });

    app.listen(
        tide_rustls::TlsListener::build()
            .addrs("0.0.0.0:4433")
            .acme(
                AcmeConfig::new()
                    .domains(vec!["domain.example".to_string()])
                    .cache_dir(path),
            ),
    )
    .await?;

    Ok(())
}
