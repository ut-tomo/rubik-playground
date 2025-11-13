#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    U, U2, Up, //順に時計回り, 180度, 反時計回り
    D, D2, Dp,
    L, L2, Lp,
    R, R2, Rp,
    F, F2, Fp,
    B, B2, Bp,
}

use crate::state::Cube;

// Corner indices (0..7):
// 0: UFL, 1: UFR, 2: UBR, 3: UBL,
// 4: DFL, 5: DFR, 6: DBR, 7: DBL

// Edge indices (0..11):
// 0: UF, 1: UR, 2: UB, 3: UL,
// 4: FL, 5: FR, 6: BR, 7: BL,
// 8: DF, 9: DR, 10: DB, 11: DL

impl Cube {
    pub fn apply_move (&mut self, m: Move) {
        match m {
            Move::U => self.apply_u(),
            Move::U2 => {self.apply_u(); self.apply_u(); }
            Move::Up => {self.apply_u(); self.apply_u(); self.apply_u(); }

            Move::D => self.apply_d(),
            Move::D2 => {self.apply_d(); self.apply_d(); }
            Move::Dp => {self.apply_d(); self.apply_d(); self.apply_d(); }

            Move::L => self.apply_l(),
            Move::L2 => {self.apply_l(); self.apply_l(); }
            Move::Lp => {self.apply_l(); self.apply_l(); self.apply_l(); }
            
            Move::R => self.apply_r(),
            Move::R2 => {self.apply_r(); self.apply_r(); }
            Move::Rp => {self.apply_r(); self.apply_r(); self.apply_r(); }

            Move::F => self.apply_f(),
            Move::F2 => {self.apply_f(); self.apply_f(); }
            Move::Fp => {self.apply_f(); self.apply_f(); self.apply_f(); }

            Move::B => self.apply_b(),
            Move::B2 => {self.apply_b(); self.apply_b(); }
            Move::Bp => {self.apply_b(); self.apply_b(); self.apply_b(); }
        }
    }

    fn apply_u(&mut self) {
        // corners: 0(UFL),1(UFR),2(UBR),3(UBL)
        let temp = self.corner_perm[0];
        self.corner_perm[0] = self.corner_perm[3];
        self.corner_perm[3] = self.corner_perm[2];
        self.corner_perm[2] = self.corner_perm[1];
        self.corner_perm[1] = temp;
        // edges: 0(UF),1(UR),2(UB),3(UL)
        let temp = self.edge_perm[0];
        self.edge_perm[0] = self.edge_perm[3];
        self.edge_perm[3] = self.edge_perm[2];
        self.edge_perm[2] = self.edge_perm[1];
        self.edge_perm[1] = temp;
        // ori不変
    }


    fn apply_d(&mut self) {
        // corners: 4(DFL),5(DFR),6(DBR),7(DBL)
        let temp = self.corner_perm[4];
        self.corner_perm[4] = self.corner_perm[7];
        self.corner_perm[7] = self.corner_perm[6];
        self.corner_perm[6] = self.corner_perm[5];
        self.corner_perm[5] = temp;
        // edges: 8(DF),9(DR),10(DB),11(DL)
        let temp = self.edge_perm[8];
        self.edge_perm[8]  = self.edge_perm[11];
        self.edge_perm[11] = self.edge_perm[10];
        self.edge_perm[10] = self.edge_perm[9];
        self.edge_perm[9]  = temp;
        // ori不変
    }


    fn apply_l(&mut self) {
        // corners: 0(UFL),3(UBL),7(DBL),4(DFL)
        let temp = self.corner_perm[0];
        self.corner_perm[0] = self.corner_perm[4];
        self.corner_perm[4] = self.corner_perm[7];
        self.corner_perm[7] = self.corner_perm[3];
        self.corner_perm[3] = temp;
        // edges: 3(UL),7(BL),11(DL),4(FL)
        let temp = self.edge_perm[3];
        self.edge_perm[3]  = self.edge_perm[4];
        self.edge_perm[4]  = self.edge_perm[11];
        self.edge_perm[11] = self.edge_perm[7];
        self.edge_perm[7]  = temp;
        // 本当は L では向きも変わるが、ここではまだ触らない
    }


    fn apply_r(&mut self) {
        // corners: 1(UFR),2(UBR),6(DBR),5(DFR)
        let temp = self.corner_perm[1];
        self.corner_perm[1] = self.corner_perm[5];
        self.corner_perm[5] = self.corner_perm[6];
        self.corner_perm[6] = self.corner_perm[2];
        self.corner_perm[2] = temp;
        // edges: 1(UR),6(BR),9(DR),5(FR)
        let temp = self.edge_perm[1];
        self.edge_perm[1] = self.edge_perm[5];
        self.edge_perm[5] = self.edge_perm[9];
        self.edge_perm[9] = self.edge_perm[6];
        self.edge_perm[6] = temp;
        // 本当は R では向きも変わるが、ここではまだ触らない
    }


    fn apply_f(&mut self) {
        // corners: 0(UFL),1(UFR),5(DFR),4(DFL)
        let temp = self.corner_perm[0];
        self.corner_perm[0] = self.corner_perm[4];
        self.corner_perm[4] = self.corner_perm[5];
        self.corner_perm[5] = self.corner_perm[1];
        self.corner_perm[1] = temp;
        // edges: 0(UF),5(FR),8(DF),4(FL)
        let temp = self.edge_perm[0];
        self.edge_perm[0] = self.edge_perm[4];
        self.edge_perm[4] = self.edge_perm[8];
        self.edge_perm[8] = self.edge_perm[5];
        self.edge_perm[5] = temp;
        // F では本当は向きも変わる（後で edge_ori, corner_ori を更新する）
    }


    fn apply_b(&mut self) {
        // corners: 2(UBR),3(UBL),7(DBL),6(DBR)
        let temp = self.corner_perm[2];
        self.corner_perm[2] = self.corner_perm[6];
        self.corner_perm[6] = self.corner_perm[7];
        self.corner_perm[7] = self.corner_perm[3];
        self.corner_perm[3] = temp;
        // edges: 2(UB),7(BL),10(DB),6(BR)
        let temp = self.edge_perm[2];
        self.edge_perm[2]  = self.edge_perm[6];
        self.edge_perm[6]  = self.edge_perm[10];
        self.edge_perm[10] = self.edge_perm[7];
        self.edge_perm[7]  = temp;
        // B では本当は向きも変わる（後で実装）
    }

    

}

/*
    fn apply_u(&mut self) {
        let cp = self.corner_perm;
        let co = self.corner_ori;
        let ep = self.edge_perm;
        let eo = self.edge_ori;

        // corners: cycle (0 1 2 3)
        self.corner_perm[0] = cp[3];
        self.corner_ori[0]  = co[3];

        self.corner_perm[1] = cp[0];
        self.corner_ori[1]  = co[0];

        self.corner_perm[2] = cp[1];
        self.corner_ori[2]  = co[1];

        self.corner_perm[3] = cp[2];
        self.corner_ori[3]  = co[2];

        // 下段の corners はそのまま
        self.corner_perm[4] = cp[4];
        self.corner_ori[4]  = co[4];
        self.corner_perm[5] = cp[5];
        self.corner_ori[5]  = co[5];
        self.corner_perm[6] = cp[6];
        self.corner_ori[6]  = co[6];
        self.corner_perm[7] = cp[7];
        self.corner_ori[7]  = co[7];

        // edges: cycle (0 1 2 3)
        self.edge_perm[0] = ep[3];
        self.edge_ori[0]  = eo[3];

        self.edge_perm[1] = ep[0];
        self.edge_ori[1]  = eo[0];

        self.edge_perm[2] = ep[1];
        self.edge_ori[2]  = eo[1];

        self.edge_perm[3] = ep[2];
        self.edge_ori[3]  = eo[2];

        // 残りの edges
        self.edge_perm[4] = ep[4];
        self.edge_ori[4]  = eo[4];
        self.edge_perm[5] = ep[5];
        self.edge_ori[5]  = eo[5];
        self.edge_perm[6] = ep[6];
        self.edge_ori[6]  = eo[6];
        self.edge_perm[7] = ep[7];
        self.edge_ori[7]  = eo[7];
        self.edge_perm[8] = ep[8];
        self.edge_ori[8]  = eo[8];
        self.edge_perm[9] = ep[9];
        self.edge_ori[9]  = eo[9];
        self.edge_perm[10] = ep[10];
        self.edge_ori[10]  = eo[10];
        self.edge_perm[11] = ep[11];
        self.edge_ori[11]  = eo[11];
    }

    // ---------- D 面：corner (4 5 6 7), edge (8 9 10 11) ----------
    // D 面側から見て時計回り

    fn apply_d(&mut self) {
        let cp = self.corner_perm;
        let co = self.corner_ori;
        let ep = self.edge_perm;
        let eo = self.edge_ori;

        // 上段 corners そのまま
        self.corner_perm[0] = cp[0];
        self.corner_ori[0]  = co[0];
        self.corner_perm[1] = cp[1];
        self.corner_ori[1]  = co[1];
        self.corner_perm[2] = cp[2];
        self.corner_ori[2]  = co[2];
        self.corner_perm[3] = cp[3];
        self.corner_ori[3]  = co[3];

        // corners: cycle (4 5 6 7)
        self.corner_perm[4] = cp[7];
        self.corner_ori[4]  = co[7];

        self.corner_perm[5] = cp[4];
        self.corner_ori[5]  = co[4];

        self.corner_perm[6] = cp[5];
        self.corner_ori[6]  = co[5];

        self.corner_perm[7] = cp[6];
        self.corner_ori[7]  = co[6];

        // 上段 edges そのまま
        self.edge_perm[0] = ep[0];
        self.edge_ori[0]  = eo[0];
        self.edge_perm[1] = ep[1];
        self.edge_ori[1]  = eo[1];
        self.edge_perm[2] = ep[2];
        self.edge_ori[2]  = eo[2];
        self.edge_perm[3] = ep[3];
        self.edge_ori[3]  = eo[3];
        self.edge_perm[4] = ep[4];
        self.edge_ori[4]  = eo[4];
        self.edge_perm[5] = ep[5];
        self.edge_ori[5]  = eo[5];
        self.edge_perm[6] = ep[6];
        self.edge_ori[6]  = eo[6];
        self.edge_perm[7] = ep[7];
        self.edge_ori[7]  = eo[7];

        // edges: cycle (8 9 10 11)
        self.edge_perm[8]  = ep[11];
        self.edge_ori[8]   = eo[11];

        self.edge_perm[9]  = ep[8];
        self.edge_ori[9]   = eo[8];

        self.edge_perm[10] = ep[9];
        self.edge_ori[10]  = eo[9];

        self.edge_perm[11] = ep[10];
        self.edge_ori[11]  = eo[10];
    }

    // ---------- L 面：corner (0 3 7 4), edge (3 7 11 4) ----------
    // L 面から見て時計回り

    fn apply_l(&mut self) {
        let cp = self.corner_perm;
        let co = self.corner_ori;
        let ep = self.edge_perm;
        let eo = self.edge_ori;

        // corners: cycle (0 3 7 4) with twist [+1,+2,+1,+2]
        self.corner_perm[0] = cp[4];
        self.corner_ori[0]  = (co[4] + 1) % 3;

        self.corner_perm[3] = cp[0];
        self.corner_ori[3]  = (co[0] + 2) % 3;

        self.corner_perm[7] = cp[3];
        self.corner_ori[7]  = (co[3] + 1) % 3;

        self.corner_perm[4] = cp[7];
        self.corner_ori[4]  = (co[7] + 2) % 3;

        // 他の corners はそのまま
        self.corner_perm[1] = cp[1];
        self.corner_ori[1]  = co[1];
        self.corner_perm[2] = cp[2];
        self.corner_ori[2]  = co[2];
        self.corner_perm[5] = cp[5];
        self.corner_ori[5]  = co[5];
        self.corner_perm[6] = cp[6];
        self.corner_ori[6]  = co[6];

        // edges: cycle (3 7 11 4) — flip はしない
        self.edge_perm[3]  = ep[4];
        self.edge_ori[3]   = eo[4];

        self.edge_perm[7]  = ep[3];
        self.edge_ori[7]   = eo[3];

        self.edge_perm[11] = ep[7];
        self.edge_ori[11]  = eo[7];

        self.edge_perm[4]  = ep[11];
        self.edge_ori[4]   = eo[11];

        // 他の edges はそのまま
        self.edge_perm[0] = ep[0];
        self.edge_ori[0]  = eo[0];
        self.edge_perm[1] = ep[1];
        self.edge_ori[1]  = eo[1];
        self.edge_perm[2] = ep[2];
        self.edge_ori[2]  = eo[2];
        self.edge_perm[5] = ep[5];
        self.edge_ori[5]  = eo[5];
        self.edge_perm[6] = ep[6];
        self.edge_ori[6]  = eo[6];
        self.edge_perm[8] = ep[8];
        self.edge_ori[8]  = eo[8];
        self.edge_perm[9] = ep[9];
        self.edge_ori[9]  = eo[9];
        self.edge_perm[10] = ep[10];
        self.edge_ori[10]  = eo[10];
    }

    // ---------- R 面：corner (1 2 6 5), edge (1 6 9 5) ----------
    // R 面から見て時計回り

    fn apply_r(&mut self) {
        let cp = self.corner_perm;
        let co = self.corner_ori;
        let ep = self.edge_perm;
        let eo = self.edge_ori;

        // corners: cycle (1 2 6 5) with twist [+1,+2,+1,+2]
        self.corner_perm[1] = cp[5];
        self.corner_ori[1]  = (co[5] + 1) % 3;

        self.corner_perm[2] = cp[1];
        self.corner_ori[2]  = (co[1] + 2) % 3;

        self.corner_perm[6] = cp[2];
        self.corner_ori[6]  = (co[2] + 1) % 3;

        self.corner_perm[5] = cp[6];
        self.corner_ori[5]  = (co[6] + 2) % 3;

        // 他の corners はそのまま
        self.corner_perm[0] = cp[0];
        self.corner_ori[0]  = co[0];
        self.corner_perm[3] = cp[3];
        self.corner_ori[3]  = co[3];
        self.corner_perm[4] = cp[4];
        self.corner_ori[4]  = co[4];
        self.corner_perm[7] = cp[7];
        self.corner_ori[7]  = co[7];

        // edges: cycle (1 6 9 5) — flip はしない
        self.edge_perm[1] = ep[5];
        self.edge_ori[1]  = eo[5];

        self.edge_perm[6] = ep[1];
        self.edge_ori[6]  = eo[1];

        self.edge_perm[9] = ep[6];
        self.edge_ori[9]  = eo[6];

        self.edge_perm[5] = ep[9];
        self.edge_ori[5]  = eo[9];

        // 他の edges
        self.edge_perm[0] = ep[0];
        self.edge_ori[0]  = eo[0];
        self.edge_perm[2] = ep[2];
        self.edge_ori[2]  = eo[2];
        self.edge_perm[3] = ep[3];
        self.edge_ori[3]  = eo[3];
        self.edge_perm[4] = ep[4];
        self.edge_ori[4]  = eo[4];
        self.edge_perm[7] = ep[7];
        self.edge_ori[7]  = eo[7];
        self.edge_perm[8] = ep[8];
        self.edge_ori[8]  = eo[8];
        self.edge_perm[10] = ep[10];
        self.edge_ori[10]  = eo[10];
        self.edge_perm[11] = ep[11];
        self.edge_ori[11]  = eo[11];
    }

    // ---------- F 面：corner (0 1 5 4), edge (0 5 8 4) ----------
    // F 面から見て時計回り

    fn apply_f(&mut self) {
        let cp = self.corner_perm;
        let co = self.corner_ori;
        let ep = self.edge_perm;
        let eo = self.edge_ori;

        // corners: cycle (0 1 5 4) with twist [+1,+2,+1,+2]
        self.corner_perm[0] = cp[4];
        self.corner_ori[0]  = (co[4] + 1) % 3;

        self.corner_perm[1] = cp[0];
        self.corner_ori[1]  = (co[0] + 2) % 3;

        self.corner_perm[5] = cp[1];
        self.corner_ori[5]  = (co[1] + 1) % 3;

        self.corner_perm[4] = cp[5];
        self.corner_ori[4]  = (co[5] + 2) % 3;

        // 他の corners はそのまま
        self.corner_perm[2] = cp[2];
        self.corner_ori[2]  = co[2];
        self.corner_perm[3] = cp[3];
        self.corner_ori[3]  = co[3];
        self.corner_perm[6] = cp[6];
        self.corner_ori[6]  = co[6];
        self.corner_perm[7] = cp[7];
        self.corner_ori[7]  = co[7];

        // edges: cycle (0 5 8 4) — これらは flip する
        self.edge_perm[0] = ep[4];
        self.edge_ori[0]  = eo[4] ^ 1;

        self.edge_perm[5] = ep[0];
        self.edge_ori[5]  = eo[0] ^ 1;

        self.edge_perm[8] = ep[5];
        self.edge_ori[8]  = eo[5] ^ 1;

        self.edge_perm[4] = ep[8];
        self.edge_ori[4]  = eo[8] ^ 1;

        // 残りの edges
        self.edge_perm[1] = ep[1];
        self.edge_ori[1]  = eo[1];
        self.edge_perm[2] = ep[2];
        self.edge_ori[2]  = eo[2];
        self.edge_perm[3] = ep[3];
        self.edge_ori[3]  = eo[3];
        self.edge_perm[6] = ep[6];
        self.edge_ori[6]  = eo[6];
        self.edge_perm[7] = ep[7];
        self.edge_ori[7]  = eo[7];
        self.edge_perm[9] = ep[9];
        self.edge_ori[9]  = eo[9];
        self.edge_perm[10] = ep[10];
        self.edge_ori[10]  = eo[10];
        self.edge_perm[11] = ep[11];
        self.edge_ori[11]  = eo[11];
    }

    // ---------- B 面：corner (3 2 6 7), edge (2 6 10 7) ----------
    // B 面から見て時計回り

    fn apply_b(&mut self) {
        let cp = self.corner_perm;
        let co = self.corner_ori;
        let ep = self.edge_perm;
        let eo = self.edge_ori;

        // corners: cycle (3 2 6 7) with twist [+1,+2,+1,+2]
        self.corner_perm[3] = cp[7];
        self.corner_ori[3]  = (co[7] + 1) % 3;

        self.corner_perm[2] = cp[3];
        self.corner_ori[2]  = (co[3] + 2) % 3;

        self.corner_perm[6] = cp[2];
        self.corner_ori[6]  = (co[2] + 1) % 3;

        self.corner_perm[7] = cp[6];
        self.corner_ori[7]  = (co[6] + 2) % 3;

        // 他の corners
        self.corner_perm[0] = cp[0];
        self.corner_ori[0]  = co[0];
        self.corner_perm[1] = cp[1];
        self.corner_ori[1]  = co[1];
        self.corner_perm[4] = cp[4];
        self.corner_ori[4]  = co[4];
        self.corner_perm[5] = cp[5];
        self.corner_ori[5]  = co[5];

        // edges: cycle (2 6 10 7) — これらは flip する
        self.edge_perm[2]  = ep[7];
        self.edge_ori[2]   = eo[7] ^ 1;

        self.edge_perm[6]  = ep[2];
        self.edge_ori[6]   = eo[2] ^ 1;

        self.edge_perm[10] = ep[6];
        self.edge_ori[10]  = eo[6] ^ 1;

        self.edge_perm[7]  = ep[10];
        self.edge_ori[7]   = eo[10] ^ 1;

        // 他の edges
        self.edge_perm[0] = ep[0];
        self.edge_ori[0]  = eo[0];
        self.edge_perm[1] = ep[1];
        self.edge_ori[1]  = eo[1];
        self.edge_perm[3] = ep[3];
        self.edge_ori[3]  = eo[3];
        self.edge_perm[4] = ep[4];
        self.edge_ori[4]  = eo[4]; // F で上書き済
        self.edge_perm[5] = ep[5];
        self.edge_ori[5]  = eo[5];
        self.edge_perm[8] = ep[8];
        self.edge_ori[8]  = eo[8];
        self.edge_perm[9] = ep[9];
        self.edge_ori[9]  = eo[9];
        self.edge_perm[11] = ep[11];
        self.edge_ori[11]  = eo[11];
    }
}
ざっくりまとめ
これで
corner_perm, edge_perm はちゃんと各面 4-cycle
corner_ori は L/R/F/B で twist（+1,+2,+1,+2）
edge_ori は F/B で flip（XOR 1）
各 move ごとに
corner twist 総和 mod 3
edge flip 総和 mod 2
が保存されるようになってるので、**合法状態判定（legality）**にも使える。
次にやると面白いのは：
Cube::legality() を実装して「ありえない状態」を検出
apply_move によるランダムスクランブル→ legality チェック
cycles.rs で 3-cycle 分解して「このアルゴリズムは corner の 3-cycle + edge の 3-cycle だよ」みたいに表示
 */