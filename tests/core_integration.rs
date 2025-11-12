//! ワークスペース統合テスト

use cube_core::{Algorithm, Cube, GroupOp, Move};

#[test]
fn test_cube_solved_state() {
    let cube = Cube::solved();
    assert!(cube.is_solved());
}

#[test]
fn test_algorithm_identity() {
    let algo = Algorithm::identity();
    assert!(algo.is_empty());
    assert_eq!(algo.to_notation(), "");
}

#[test]
fn test_algorithm_inverse_composition() {
    let algo = Algorithm::new(vec![Move::R, Move::U, Move::RPrime, Move::UPrime]);
    let inverse = algo.inverse();
    let composed = algo.compose(&inverse);

    // A * A^-1 = I（単位元）
    // 手順の長さは元の2倍になる（簡約化は未実装）
    assert_eq!(composed.len(), algo.len() * 2);
}

#[test]
fn test_move_inverse() {
    assert_eq!(Move::R.inverse(), Move::RPrime);
    assert_eq!(Move::RPrime.inverse(), Move::R);
    assert_eq!(Move::R2.inverse(), Move::R2);
}

#[test]
fn test_algorithm_power_zero() {
    let algo = Algorithm::new(vec![Move::R, Move::U]);
    let power_zero = algo.power(0);

    assert!(power_zero.is_empty());
}

#[test]
fn test_algorithm_power_negative() {
    let algo = Algorithm::new(vec![Move::R, Move::U]);
    let power_minus_one = algo.power(-1);

    assert_eq!(power_minus_one.moves(), algo.inverse().moves());
}

#[test]
fn test_move_notation_parsing() {
    assert_eq!(Move::from_notation("R"), Some(Move::R));
    assert_eq!(Move::from_notation("R'"), Some(Move::RPrime));
    assert_eq!(Move::from_notation("R2"), Some(Move::R2));
    assert_eq!(Move::from_notation("Invalid"), None);
}

#[test]
fn test_all_moves_count() {
    let moves = Move::all_moves();
    assert_eq!(moves.len(), 18);
}
