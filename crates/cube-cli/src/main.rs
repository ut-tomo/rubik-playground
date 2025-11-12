//! ルービックキューブCLI
//!
//! 群演算の検証、スクランブル生成、アルゴリズムのテストなどを行うCLI

use clap::{Parser, Subcommand};
use cube_core::{Algorithm, Cube, GroupOp, Move};

#[derive(Parser)]
#[command(name = "cube-cli")]
#[command(about = "ルービックキューブ群演算CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// キューブの状態を表示
    Show,

    /// アルゴリズムを実行
    Apply {
        /// 実行する手順（例: "R U R' U'"）
        #[arg(value_name = "ALGORITHM")]
        algorithm: String,
    },

    /// アルゴリズムの逆手を計算
    Inverse {
        /// 手順（例: "R U R' U'"）
        #[arg(value_name = "ALGORITHM")]
        algorithm: String,
    },

    /// 2つのアルゴリズムを合成
    Compose {
        /// 最初のアルゴリズム
        #[arg(value_name = "ALGO1")]
        algo1: String,

        /// 2番目のアルゴリズム
        #[arg(value_name = "ALGO2")]
        algo2: String,
    },

    /// アルゴリズムのべき乗を計算
    Power {
        /// アルゴリズム
        #[arg(value_name = "ALGORITHM")]
        algorithm: String,

        /// 指数
        #[arg(value_name = "N")]
        n: i32,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Show => {
            let cube = Cube::solved();
            println!("解決済みキューブ:");
            println!("{}", cube);
        }

        Commands::Apply { algorithm } => {
            let cube = Cube::solved();
            let algo = parse_algorithm(&algorithm);

            println!("アルゴリズム: {}", algo.to_notation());
            let result = algo.apply(&cube);
            println!("結果:");
            println!("{}", result);
        }

        Commands::Inverse { algorithm } => {
            let algo = parse_algorithm(&algorithm);
            let inv = algo.inverse();

            println!("元のアルゴリズム: {}", algo.to_notation());
            println!("逆手: {}", inv.to_notation());
        }

        Commands::Compose { algo1, algo2 } => {
            let a1 = parse_algorithm(&algo1);
            let a2 = parse_algorithm(&algo2);
            let composed = a1.compose(&a2);

            println!("アルゴリズム1: {}", a1.to_notation());
            println!("アルゴリズム2: {}", a2.to_notation());
            println!("合成結果: {}", composed.to_notation());
        }

        Commands::Power { algorithm, n } => {
            let algo = parse_algorithm(&algorithm);
            let powered = algo.power(n);

            println!("アルゴリズム: {}", algo.to_notation());
            println!("{}乗: {}", n, powered.to_notation());
        }
    }
}

/// 文字列からアルゴリズムをパース
fn parse_algorithm(s: &str) -> Algorithm {
    let moves: Vec<Move> = s
        .split_whitespace()
        .filter_map(Move::from_notation)
        .collect();

    Algorithm::new(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_algorithm() {
        let algo = parse_algorithm("R U R' U'");
        assert_eq!(algo.len(), 4);
        assert_eq!(algo.to_notation(), "R U R' U'");
    }
}
