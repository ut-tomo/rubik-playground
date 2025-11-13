use cube_core::{
    apply_alg, commutator, conjugate, corner_cycles, edge_cycles, invert_alg, Cube, Move,
};

fn main() {
    println!("🧊 Rubik's Cube Playground - Demo\n");
    println!("{}", "=".repeat(50));

    demo_basic_moves();
    demo_algorithm_operations();
    demo_group_theory();
    demo_legality_check();
    demo_cycle_decomposition();

    println!("\n{}", "=".repeat(50));
    println!("✅ All demos completed!");
}

fn demo_basic_moves() {
    println!("\n Demo 1: Basic Moves");
    println!("{}", "-".repeat(50));

    let mut cube = Cube::identity();
    println!("Starting state: solved = {}", cube.is_solved());

    // Apply single move
    cube.apply_move(Move::R);
    println!("After R move: solved = {}", cube.is_solved());

    // Apply inverse to restore
    cube.apply_move(Move::Rp);
    println!("After R' move: solved = {}", cube.is_solved());
}

fn demo_algorithm_operations() {
    println!("\n Demo 2: Algorithm Operations");
    println!("{}", "-".repeat(50));

    // Sexy move: R U R' U'
    let sexy_move = vec![Move::R, Move::U, Move::Rp, Move::Up];
    println!("Algorithm: R U R' U' (Sexy Move)");
    println!("  Length: {} moves", sexy_move.len());

    // Apply algorithm
    let mut cube = Cube::identity();
    cube = apply_alg(&cube, &sexy_move);
    println!("  After applying: solved = {}", cube.is_solved());

    // Calculate and apply inverse
    let inverse = invert_alg(&sexy_move);
    println!("\nInverse algorithm: U R U' R'");
    println!("  Length: {} moves", inverse.len());
    cube = apply_alg(&cube, &inverse);
    println!("  After inverse: solved = {}", cube.is_solved());
}

fn demo_group_theory() {
    println!("\n Demo 3: Group Theory Operations");
    println!("{}", "-".repeat(50));

    let a = vec![Move::R];
    let b = vec![Move::U];

    // Commutator: [A, B] = A B A' B'
    let comm = commutator(&a, &b);
    println!("Commutator [R, U] = R U R' U'");
    println!("  Length: {} moves", comm.len());

    let _cube = apply_alg(&Cube::identity(), &comm);
    println!("  Affects: corners only (typical for [R,U])");

    // Conjugate: A B A'
    let conj = conjugate(&a, &b);
    println!("\nConjugate R U R' = R (U) R'");
    println!("  Length: {} moves", conj.len());
}

fn demo_legality_check() {
    println!("\n Demo 4: Legality Check");
    println!("{}", "-".repeat(50));

    // Legal state (solved)
    let cube = Cube::identity();
    let info = cube.legality();
    println!("Solved cube:");
    println!("  Corner parity: {}", info.corner_parity);
    println!("  Edge parity: {}", info.edge_parity);
    println!("  Edge flip sum (mod 2): {}", info.edge_flip_sum_mod2);
    println!("  Corner twist sum (mod 3): {}", info.corner_twist_sum_mod3);
    println!("  Is legal: {}", info.is_legal);

    // After some moves
    let cube2 = apply_alg(&Cube::identity(), &[Move::R, Move::U, Move::Rp, Move::Up]);
    let info2 = cube2.legality();
    println!("\nAfter R U R' U':");
    println!("  Corner parity: {}", info2.corner_parity);
    println!("  Edge parity: {}", info2.edge_parity);
    println!("  Is legal: {}", info2.is_legal);
}

fn demo_cycle_decomposition() {
    println!("\n Demo 5: Cycle Decomposition");
    println!("{}", "-".repeat(50));

    let alg = vec![Move::R, Move::U, Move::Rp, Move::Up];
    let cube = apply_alg(&Cube::identity(), &alg);

    let c_cycles = corner_cycles(&cube);
    let e_cycles = edge_cycles(&cube);

    println!("After R U R' U' (Sexy Move):");
    println!("  Corner cycles: {:?}", c_cycles);
    if c_cycles.len() == 1 && c_cycles[0].len() == 3 {
        println!("    → 3-cycle detected (swaps 3 corners)");
    }
    println!("  Edge cycles: {:?}", e_cycles);
    if e_cycles.is_empty() || (e_cycles.len() == 1 && e_cycles[0].len() == 1) {
        println!("    → Edges unchanged");
    }
}
