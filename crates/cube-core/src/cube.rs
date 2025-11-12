//! ルービックキューブの状態表現

use serde::{Deserialize, Serialize};

/// 3x3x3 ルービックキューブの状態
/// 
/// 状態は24個の要素で表現:
/// - 8個のコーナーピース（各3つの向き）
/// - 12個のエッジピース（各2つの向き）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cube {
    /// コーナーピースの位置 (0-7)
    corners: [u8; 8],
    /// コーナーピースの向き (0-2)
    corner_orientations: [u8; 8],
    /// エッジピースの位置 (0-11)
    edges: [u8; 12],
    /// エッジピースの向き (0-1)
    edge_orientations: [u8; 12],
}

impl Cube {
    /// 解決済み状態のキューブを作成
    pub fn solved() -> Self {
        Self {
            corners: [0, 1, 2, 3, 4, 5, 6, 7],
            corner_orientations: [0; 8],
            edges: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            edge_orientations: [0; 12],
        }
    }

    /// キューブが解決済み状態かチェック
    pub fn is_solved(&self) -> bool {
        *self == Self::solved()
    }

    /// コーナーピースの取得
    pub fn corners(&self) -> &[u8; 8] {
        &self.corners
    }

    /// エッジピースの取得
    pub fn edges(&self) -> &[u8; 12] {
        &self.edges
    }

    /// 状態の文字列表現
    pub fn to_string(&self) -> String {
        format!(
            "Corners: {:?}\nEdges: {:?}",
            self.corners, self.edges
        )
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::solved()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved_state() {
        let cube = Cube::solved();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_clone() {
        let cube1 = Cube::solved();
        let cube2 = cube1.clone();
        assert_eq!(cube1, cube2);
    }
}