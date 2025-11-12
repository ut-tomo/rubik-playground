//! WebAssembly公開インターフェース

use cube_core::{Algorithm, Cube, GroupOp, Move};
use wasm_bindgen::prelude::*;

/// WASMから利用可能なキューブラッパー
#[wasm_bindgen]
pub struct WasmCube {
    cube: Cube,
}

impl Default for WasmCube {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl WasmCube {
    /// 解決済みキューブを作成
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cube: Cube::solved(),
        }
    }

    /// キューブが解決済みかチェック
    #[wasm_bindgen(js_name = isSolved)]
    pub fn is_solved(&self) -> bool {
        self.cube.is_solved()
    }

    /// 状態を文字列で取得
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string_js(&self) -> String {
        self.cube.to_string()
    }
}

/// WASMから利用可能なアルゴリズムラッパー
#[wasm_bindgen]
pub struct WasmAlgorithm {
    algo: Algorithm,
}

#[wasm_bindgen]
impl WasmAlgorithm {
    /// 文字列からアルゴリズムを作成
    #[wasm_bindgen(constructor)]
    pub fn new(notation: &str) -> Result<WasmAlgorithm, JsValue> {
        let moves: Vec<Move> = notation
            .split_whitespace()
            .filter_map(Move::from_notation)
            .collect();

        if moves.is_empty() && !notation.trim().is_empty() {
            return Err(JsValue::from_str("Invalid algorithm notation"));
        }

        Ok(Self {
            algo: Algorithm::new(moves),
        })
    }

    /// 表記を取得
    #[wasm_bindgen(js_name = getNotation)]
    pub fn get_notation(&self) -> String {
        self.algo.to_notation()
    }

    /// 逆手を取得
    #[wasm_bindgen(js_name = inverse)]
    pub fn inverse(&self) -> WasmAlgorithm {
        Self {
            algo: self.algo.inverse(),
        }
    }

    /// 合成
    #[wasm_bindgen(js_name = compose)]
    pub fn compose(&self, other: &WasmAlgorithm) -> WasmAlgorithm {
        Self {
            algo: self.algo.compose(&other.algo),
        }
    }

    /// べき乗
    #[wasm_bindgen(js_name = power)]
    pub fn power(&self, n: i32) -> WasmAlgorithm {
        Self {
            algo: self.algo.power(n),
        }
    }

    /// キューブに適用
    #[wasm_bindgen(js_name = apply)]
    pub fn apply(&self, cube: &WasmCube) -> WasmCube {
        WasmCube {
            cube: self.algo.apply(&cube.cube),
        }
    }
}

/// モジュール初期化
#[wasm_bindgen(start)]
pub fn main() {
    // パニックフックの設定などはここに追加可能
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_wasm_cube_creation() {
        let cube = WasmCube::new();
        assert!(cube.is_solved());
    }

    #[wasm_bindgen_test]
    fn test_wasm_algorithm() {
        let algo = WasmAlgorithm::new("R U R' U'").unwrap();
        assert_eq!(algo.get_notation(), "R U R' U'");
    }
}
