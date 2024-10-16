use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct UAM {
    id: u32,
    time_boot_ms: u32,
    lat: f64,
    lon: f64,
    alt: f64,
    relative_alt: f32,
    vx: f32,
    vy: f32,
    vz: f32,
    hdg: f32,
}

#[wasm_bindgen]
pub struct UAMs {
    uams: Vec<UAM>,
}