use cube_core::{Cube, Move};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Console logging for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// JavaScript-friendly cube representation
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmCube {
    inner: Cube,
}

impl Default for WasmCube {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl WasmCube {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_log!("Creating new cube");
        Self {
            inner: Cube::identity(),
        }
    }

    #[wasm_bindgen(js_name = identity)]
    pub fn identity() -> Self {
        Self {
            inner: Cube::identity(),
        }
    }

    #[wasm_bindgen(js_name = isSolved)]
    pub fn is_solved(&self) -> bool {
        self.inner.is_solved()
    }

    #[wasm_bindgen(js_name = applyMove)]
    pub fn apply_move(&mut self, move_str: &str) -> Result<(), JsValue> {
        let m = parse_move(move_str)
            .ok_or_else(|| JsValue::from_str(&format!("Invalid move: {}", move_str)))?;
        self.inner.apply_move(m);
        Ok(())
    }

    #[wasm_bindgen(js_name = applyAlgorithm)]
    pub fn apply_algorithm(&mut self, alg_str: &str) -> Result<(), JsValue> {
        let moves = parse_algorithm(alg_str)?;
        for m in moves {
            self.inner.apply_move(m);
        }
        Ok(())
    }

    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> JsValue {
        let state = CubeState::from_cube(&self.inner);
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    #[wasm_bindgen(js_name = getLegality)]
    pub fn get_legality(&self) -> JsValue {
        let info = self.inner.legality();
        let legality = LegalityData {
            corner_parity: info.corner_parity,
            edge_parity: info.edge_parity,
            edge_flip_sum_mod2: info.edge_flip_sum_mod2,
            corner_twist_sum_mod3: info.corner_twist_sum_mod3,
            is_legal: info.is_legal,
        };
        serde_wasm_bindgen::to_value(&legality).unwrap()
    }

    #[wasm_bindgen(js_name = getCornerCycles)]
    pub fn get_corner_cycles(&self) -> JsValue {
        let cycles = cube_core::corner_cycles(&self.inner);
        serde_wasm_bindgen::to_value(&cycles).unwrap()
    }

    #[wasm_bindgen(js_name = getEdgeCycles)]
    pub fn get_edge_cycles(&self) -> JsValue {
        let cycles = cube_core::edge_cycles(&self.inner);
        serde_wasm_bindgen::to_value(&cycles).unwrap()
    }
}

// Helper functions
#[wasm_bindgen(js_name = invertAlgorithm)]
pub fn invert_algorithm(alg_str: &str) -> Result<String, JsValue> {
    let moves = parse_algorithm(alg_str)?;
    let inverted = cube_core::invert_alg(&moves);
    Ok(format_algorithm(&inverted))
}

#[wasm_bindgen(js_name = commutator)]
pub fn commutator(a_str: &str, b_str: &str) -> Result<String, JsValue> {
    let a = parse_algorithm(a_str)?;
    let b = parse_algorithm(b_str)?;
    let result = cube_core::commutator(&a, &b);
    Ok(format_algorithm(&result))
}

#[wasm_bindgen(js_name = conjugate)]
pub fn conjugate(a_str: &str, b_str: &str) -> Result<String, JsValue> {
    let a = parse_algorithm(a_str)?;
    let b = parse_algorithm(b_str)?;
    let result = cube_core::conjugate(&a, &b);
    Ok(format_algorithm(&result))
}

// Serializable types for JavaScript
#[derive(Serialize, Deserialize)]
struct CubeState {
    corner_perm: Vec<u8>,
    corner_ori: Vec<u8>,
    edge_perm: Vec<u8>,
    edge_ori: Vec<u8>,
}

impl CubeState {
    fn from_cube(cube: &Cube) -> Self {
        Self {
            corner_perm: cube.corner_perm.to_vec(),
            corner_ori: cube.corner_ori.to_vec(),
            edge_perm: cube.edge_perm.to_vec(),
            edge_ori: cube.edge_ori.to_vec(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LegalityData {
    corner_parity: i32,
    edge_parity: i32,
    edge_flip_sum_mod2: u8,
    corner_twist_sum_mod3: u8,
    is_legal: bool,
}

// Parse move notation
fn parse_move(s: &str) -> Option<Move> {
    match s.trim() {
        "U" => Some(Move::U),
        "U2" => Some(Move::U2),
        "U'" | "Up" => Some(Move::Up),
        "D" => Some(Move::D),
        "D2" => Some(Move::D2),
        "D'" | "Dp" => Some(Move::Dp),
        "L" => Some(Move::L),
        "L2" => Some(Move::L2),
        "L'" | "Lp" => Some(Move::Lp),
        "R" => Some(Move::R),
        "R2" => Some(Move::R2),
        "R'" | "Rp" => Some(Move::Rp),
        "F" => Some(Move::F),
        "F2" => Some(Move::F2),
        "F'" | "Fp" => Some(Move::Fp),
        "B" => Some(Move::B),
        "B2" => Some(Move::B2),
        "B'" | "Bp" => Some(Move::Bp),
        _ => None,
    }
}

fn parse_algorithm(s: &str) -> Result<Vec<Move>, JsValue> {
    s.split_whitespace()
        .map(|token| {
            parse_move(token).ok_or_else(|| JsValue::from_str(&format!("Invalid move: {}", token)))
        })
        .collect()
}

fn format_algorithm(moves: &[Move]) -> String {
    moves
        .iter()
        .map(|m| format!("{:?}", m))
        .collect::<Vec<_>>()
        .join(" ")
}
