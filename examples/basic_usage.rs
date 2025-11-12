//! 基本的な使用例

use cube_core::{Algorithm, Cube, GroupOp, Move};

fn main() {
    println!("=== ルービックキューブ群演算の基本例 ===\n");

    // 1. 解決済みキューブの作成
    let cube = Cube::solved();
    println!("1. 解決済みキューブ:");
    println!("{}\n", cube);

    // 2. アルゴリズムの作成
    let sexy_move = Algorithm::new(vec![Move::R, Move::U, Move::RPrime, Move::UPrime]);
    println!("2. セクシームーブ: {}\n", sexy_move.to_notation());

    // 3. 逆手の計算
    let inverse = sexy_move.inverse();
    println!("3. 逆手: {}\n", inverse.to_notation());

    // 4. アルゴリズムの合成
    let composed = sexy_move.compose(&inverse);
    println!("4. 合成（A * A^-1）: {}", composed.to_notation());
    println!("   → これは単位元になるはず\n");

    // 5. べき乗
    let sexy_x6 = sexy_move.power(6);
    println!("5. セクシームーブの6乗:");
    println!(
        "   {} = {}\n",
        sexy_move.to_notation(),
        sexy_x6.to_notation()
    );

    // 6. 手の表記変換
    println!("6. 手の表記例:");
    for m in [Move::R, Move::RPrime, Move::R2, Move::U, Move::UPrime] {
        println!("   {:?} -> {}", m, m.to_notation());
    }
}
