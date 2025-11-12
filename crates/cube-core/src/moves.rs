//! ルービックキューブの基本操作（手）

use serde::{Deserialize, Serialize};

/// ルービックキューブの基本的な手
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Move {
    /// 上面を時計回りに90度
    U,
    /// 上面を反時計回りに90度
    UPrime,
    /// 上面を180度
    U2,
    
    /// 下面を時計回りに90度
    D,
    /// 下面を反時計回りに90度
    DPrime,
    /// 下面を180度
    D2,
    
    /// 右面を時計回りに90度
    R,
    /// 右面を反時計回りに90度
    RPrime,
    /// 右面を180度
    R2,
    
    /// 左面を時計回りに90度
    L,
    /// 左面を反時計回りに90度
    LPrime,
    /// 左面を180度
    L2,
    
    /// 前面を時計回りに90度
    F,
    /// 前面を反時計回りに90度
    FPrime,
    /// 前面を180度
    F2,
    
    /// 後面を時計回りに90度
    B,
    /// 後面を反時計回りに90度
    BPrime,
    /// 後面を180度
    B2,
}

impl Move {
    /// 手を文字列表記に変換
    pub fn to_notation(&self) -> &'static str {
        match self {
            Move::U => "U",
            Move::UPrime => "U'",
            Move::U2 => "U2",
            Move::D => "D",
            Move::DPrime => "D'",
            Move::D2 => "D2",
            Move::R => "R",
            Move::RPrime => "R'",
            Move::R2 => "R2",
            Move::L => "L",
            Move::LPrime => "L'",
            Move::L2 => "L2",
            Move::F => "F",
            Move::FPrime => "F'",
            Move::F2 => "F2",
            Move::B => "B",
            Move::BPrime => "B'",
            Move::B2 => "B2",
        }
    }

    /// 文字列表記から手を解析
    pub fn from_notation(s: &str) -> Option<Self> {
        match s {
            "U" => Some(Move::U),
            "U'" | "U-" => Some(Move::UPrime),
            "U2" => Some(Move::U2),
            "D" => Some(Move::D),
            "D'" | "D-" => Some(Move::DPrime),
            "D2" => Some(Move::D2),
            "R" => Some(Move::R),
            "R'" | "R-" => Some(Move::RPrime),
            "R2" => Some(Move::R2),
            "L" => Some(Move::L),
            "L'" | "L-" => Some(Move::LPrime),
            "L2" => Some(Move::L2),
            "F" => Some(Move::F),
            "F'" | "F-" => Some(Move::FPrime),
            "F2" => Some(Move::F2),
            "B" => Some(Move::B),
            "B'" | "B-" => Some(Move::BPrime),
            "B2" => Some(Move::B2),
            _ => None,
        }
    }

    /// 逆手を取得
    pub fn inverse(&self) -> Self {
        match self {
            Move::U => Move::UPrime,
            Move::UPrime => Move::U,
            Move::U2 => Move::U2,
            Move::D => Move::DPrime,
            Move::DPrime => Move::D,
            Move::D2 => Move::D2,
            Move::R => Move::RPrime,
            Move::RPrime => Move::R,
            Move::R2 => Move::R2,
            Move::L => Move::LPrime,
            Move::LPrime => Move::L,
            Move::L2 => Move::L2,
            Move::F => Move::FPrime,
            Move::FPrime => Move::F,
            Move::F2 => Move::F2,
            Move::B => Move::BPrime,
            Move::BPrime => Move::B,
            Move::B2 => Move::B2,
        }
    }

    /// すべての基本手を列挙
    pub fn all_moves() -> [Move; 18] {
        [
            Move::U, Move::UPrime, Move::U2,
            Move::D, Move::DPrime, Move::D2,
            Move::R, Move::RPrime, Move::R2,
            Move::L, Move::LPrime, Move::L2,
            Move::F, Move::FPrime, Move::F2,
            Move::B, Move::BPrime, Move::B2,
        ]
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_notation())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_notation() {
        assert_eq!(Move::U.to_notation(), "U");
        assert_eq!(Move::RPrime.to_notation(), "R'");
        assert_eq!(Move::F2.to_notation(), "F2");
    }

    #[test]
    fn test_from_notation() {
        assert_eq!(Move::from_notation("U"), Some(Move::U));
        assert_eq!(Move::from_notation("R'"), Some(Move::RPrime));
        assert_eq!(Move::from_notation("F2"), Some(Move::F2));
        assert_eq!(Move::from_notation("X"), None);
    }

    #[test]
    fn test_inverse() {
        assert_eq!(Move::U.inverse(), Move::UPrime);
        assert_eq!(Move::RPrime.inverse(), Move::R);
        assert_eq!(Move::F2.inverse(), Move::F2);
    }
}