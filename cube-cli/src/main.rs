use cube_core::{Cube, Move, apply_alg, commutator};
use cube_core::{cycles::corner_cycles, legality::LegalityInfo}; // lib.rs から re-export してもOK

fn main() {
    // とりあえずテスト：M = [R, U, R', U']
    let alg = vec![Move::R, Move::U, Move::Rp, Move::Up];

    let solved = Cube::identity();
    let cube = apply_alg(&solved, &alg);

    // cycle 分解
    let c_cycles = corner_cycles(&cube);
    println!("corner cycles: {:?}", c_cycles);

    // legality
    let info = cube.legality();
    println!("legal? {}", info.is_legal);

    // commutator test: [R, U]
    let comm = commutator(&[Move::R], &[Move::U]);
    println!("commutator [R,U] length = {}", comm.len());
}
