//! # cube-core
//!
//! ルービックキューブの状態表現と群演算を提供するコアライブラリ

pub mod cube;
pub mod group;
pub mod moves;

pub use cube::Cube;
pub use group::GroupOp;
pub use moves::Move;

// 追加の公開型
pub use group::Algorithm;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let cube = Cube::solved();
        assert!(cube.is_solved());
    }
}
