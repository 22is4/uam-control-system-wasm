use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, serde::Serialize)]
pub struct UAM {
    lat: f64,
    lon: f64,
    alt: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

#[wasm_bindgen]
pub struct UAMs {
    uams: Vec<UAM>,
}

#[wasm_bindgen]
impl UAM {
    pub fn new(lat: f64, lon: f64, alt: f64, vx: f64, vy: f64, vz: f64) -> UAM {
        UAM {
            lat,
            lon,
            alt,
            vx,
            vy,
            vz,
        }
    }

    pub fn predict_collision(&self, other: &UAM) -> Option<f64> {
        let dx = self.lat - other.lat;
        let dy = self.lon - other.lon;
        let dz = self.alt - other.alt;
        let dvx = self.vx - other.vx;
        let dvy = self.vy - other.vy;
        let dvz = self.vz - other.vz;
        let a = dvx * dvx + dvy * dvy + dvz * dvz; // dot product of dv and dv
        let b = 2.0 * (dx * dvx + dy * dvy + dz * dvz); // dot product of d and dv
        let c = dx * dx + dy * dy + dz * dz; // dot product of d and d
        let d = b * b - 4.0 * a * c; // discriminant

        if d < 0.0 {
            return None;
        }

        let t = (-b - d.sqrt()) / (2.0 * a);
        if t < 0.0 {
            return None;
        }

        Some(t)
    }
}

#[wasm_bindgen]
impl UAMs {
    pub fn new() -> UAMs {
        UAMs { uams: Vec::new() }
    }

    pub fn add(&mut self, uam: UAM) {
        self.uams.push(uam);
    }

    pub fn predict_collisions(&self) -> JsValue {
        let mut collisions = Vec::new();
        for i in 0..self.uams.len() {
            for j in i + 1..self.uams.len() {
                if let Some(_t) = self.uams[i].predict_collision(&self.uams[j]) {
                    collisions.push((self.uams[i].clone(), self.uams[j].clone()));
                }
            }
        }
        to_value(&collisions).unwrap()
    }
}
