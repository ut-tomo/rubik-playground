use cube_core::{Cube, Move, apply_alg, invert_alg, commutator};//! Basic usage example of the Rubik's Cube library//! 基本的な使用例



fn main() {

    println!("=== Rubik's Cube Playground ===\n");

use cube_core::{Cube, Move, apply_alg, invert_alg, commutator};use cube_core::{Algorithm, Cube, GroupOp, Move};

    let mut cube = Cube::identity();

    println!("Starting with solved cube: {}", cube.is_solved());



    println!("\nApplying moves: R, U, R', U'");fn main() {fn main() {

    let alg = vec![Move::R, Move::U, Move::Rp, Move::Up];

    apply_alg(&mut cube, &alg);    println!("=== Rubik's Cube Playground ===\n");    println!("=== ルービックキューブ群演算の基本例 ===\n");

    println!("After moves: {}", cube.is_solved());



    println!("\nApplying inverse algorithm");

    let inv = invert_alg(&alg);    // Create a solved cube    // 1. 解決済みキューブの作成

    apply_alg(&mut cube, &inv);

    println!("After inverse: {}", cube.is_solved());    let mut cube = Cube::identity();    let cube = Cube::solved();



    println!("\n=== Commutator Example ===");    println!("Starting with solved cube: {}", cube.is_solved());    println!("1. 解決済みキューブ:");

    let mut cube2 = Cube::identity();

    let a = vec![Move::R, Move::U];    println!("{}\n", cube);

    let b = vec![Move::L, Move::D];

    let comm = commutator(&a, &b);    // Apply some moves

    println!("Commutator [R U, L D] has {} moves", comm.len());

    apply_alg(&mut cube2, &comm);    println!("\nApplying moves: R, U, R', U'");    // 2. アルゴリズムの作成

    

    let info = cube2.legality();    let alg = vec![Move::R, Move::U, Move::Rp, Move::Up];    let sexy_move = Algorithm::new(vec![Move::R, Move::U, Move::RPrime, Move::UPrime]);

    println!("Cube is legal: {}", info.is_legal);

        apply_alg(&mut cube, &alg);    println!("2. セクシームーブ: {}\n", sexy_move.to_notation());

    println!("\n=== Example Complete ===");

}    println!("After moves: {}", cube.is_solved());


    // 3. 逆手の計算

    // Apply inverse to solve    let inverse = sexy_move.inverse();

    println!("\nApplying inverse algorithm");    println!("3. 逆手: {}\n", inverse.to_notation());

    let inv = invert_alg(&alg);

    apply_alg(&mut cube, &inv);    // 4. アルゴリズムの合成

    println!("After inverse: {}", cube.is_solved());    let composed = sexy_move.compose(&inverse);

    println!("4. 合成（A * A^-1）: {}", composed.to_notation());

    // Demonstrate commutator    println!("   → これは単位元になるはず\n");

    println!("\n=== Commutator Example ===");

    let mut cube2 = Cube::identity();    // 5. べき乗

    let a = vec![Move::R, Move::U];    let sexy_x6 = sexy_move.power(6);

    let b = vec![Move::L, Move::D];    println!("5. セクシームーブの6乗:");

    let comm = commutator(&a, &b);    println!(

    println!("Commutator [R U, L D] has {} moves", comm.len());        "   {} = {}\n",

    apply_alg(&mut cube2, &comm);        sexy_move.to_notation(),

            sexy_x6.to_notation()

    // Check legality    );

    let info = cube2.legality();

    println!("Cube is legal: {}", info.is_legal);    // 6. 手の表記変換

        println!("6. 手の表記例:");

    println!("\n=== Example Complete ===");    for m in [Move::R, Move::RPrime, Move::R2, Move::U, Move::UPrime] {

}        println!("   {:?} -> {}", m, m.to_notation());

    }
}
