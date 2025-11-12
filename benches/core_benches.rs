//! ベンチマーク
//!
//! 実行方法: cargo bench

use cube_core::{Algorithm, Cube, GroupOp, Move};

fn main() {
    println!("=== ルービックキューブ群演算のベンチマーク ===\n");
    
    // 1. キューブの作成
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        let _ = Cube::solved();
    }
    let elapsed = start.elapsed();
    println!("キューブ作成 x10000: {:?}", elapsed);
    
    // 2. アルゴリズムの逆手計算
    let algo = Algorithm::new(vec![Move::R, Move::U, Move::RPrime, Move::UPrime]);
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        let _ = algo.inverse();
    }
    let elapsed = start.elapsed();
    println!("逆手計算 x10000: {:?}", elapsed);
    
    // 3. アルゴリズムの合成
    let algo1 = Algorithm::new(vec![Move::R, Move::U]);
    let algo2 = Algorithm::new(vec![Move::RPrime, Move::UPrime]);
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        let _ = algo1.compose(&algo2);
    }
    let elapsed = start.elapsed();
    println!("合成 x10000: {:?}", elapsed);
    
    // 4. べき乗計算
    let algo = Algorithm::new(vec![Move::R, Move::U, Move::RPrime, Move::UPrime]);
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = algo.power(6);
    }
    let elapsed = start.elapsed();
    println!("べき乗(n=6) x1000: {:?}", elapsed);
    
    println!("\n✓ ベンチマーク完了");
}
