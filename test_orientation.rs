// Test orientation correctness
// Compile: rustc --edition 2021 test_orientation.rs -L target/debug/deps -L target/debug --extern cube_core=target/debug/libcube_core.rlib

use cube_core::state::Cube;
use cube_core::moves::Move;

fn main() {
    let mut cube = Cube::new();
    
    println!("Initial state:");
    println!("corner_perm: {:?}", cube.corner_perm());
    println!("corner_ori: {:?}", cube.corner_ori());
    
    // Apply L 4 times - should return to original orientation
    println!("\nApplying L face 4 times...");
    for i in 1..=4 {
        cube.apply_move(Move::L);
        println!("\nAfter L{}:", i);
        println!("corner_perm: {:?}", cube.corner_perm());
        println!("corner_ori: {:?}", cube.corner_ori());
    }
    
    println!("\n--- Testing other faces ---");
    
    // Test R face
    let mut cube2 = Cube::new();
    println!("\nR face 4 times:");
    for i in 1..=4 {
        cube2.apply_move(Move::R);
        if i == 4 {
            println!("After R4:");
            println!("corner_ori: {:?}", cube2.corner_ori());
        }
    }
    
    // Test F face
    let mut cube3 = Cube::new();
    println!("\nF face 4 times:");
    for i in 1..=4 {
        cube3.apply_move(Move::F);
        if i == 4 {
            println!("After F4:");
            println!("corner_ori: {:?}", cube3.corner_ori());
        }
    }
    
    // Test B face
    let mut cube4 = Cube::new();
    println!("\nB face 4 times:");
    for i in 1..=4 {
        cube4.apply_move(Move::B);
        if i == 4 {
            println!("After B4:");
            println!("corner_ori: {:?}", cube4.corner_ori());
        }
    }
}
