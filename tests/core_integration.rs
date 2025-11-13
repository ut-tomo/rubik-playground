//! ワークスペース統合テスト

use cube_core::{Cube, Move, apply_alg, invert_alg, commutator, conjugate};

#[test]
fn test_cube_solved_state() {
    let cube = Cube::identity();
    assert!(cube.is_solved());
}

#[test]
fn test_apply_move() {
    let mut cube = Cube::identity();
    cube.apply_move(Move::R);
    assert!(!cube.is_solved());
}

#[test]
fn test_algorithm_inverse() {
    let mut cube = Cube::identity();
    let alg = vec![Move::R, Move::U, Move::Rp, Move::Up];
    apply_alg(&mut cube, &alg);
    let inv = invert_alg(&alg);
    apply_alg(&mut cube, &inv);
    assert!(cube.is_solved());
}

#[test]
fn test_move_inverse() {
    assert_eq!(Move::R.inverse(), Move::Rp);
    assert_eq!(Move::Rp.inverse(), Move::R);
    assert_eq!(Move::R2.inverse(), Move::R2);
}

#[test]
fn test_commutator() {
    let a = vec![Move::R, Move::U];
    let b = vec![Move::L, Move::D];
    let comm = commutator(&a, &b);
    
    // Commutator should have length = len(a) + len(b) + len(a') + len(b')
    assert_eq!(comm.len(), a.len() + b.len() + a.len() + b.len());
}

#[test]
fn test_conjugate() {
    let a = vec![Move::R];
    let b = vec![Move::U];
    let conj = conjugate(&a, &b);
    
    // Conjugate should have length = len(a) + len(b) + len(a')
    assert_eq!(conj.len(), a.len() + b.len() + a.len());
}

#[test]
fn test_legality() {
    let cube = Cube::identity();
    let info = cube.legality();
    assert!(info.is_legal);
}
