# Rubik's Cube Playground

ルービックキューブの群演算を探索するための Rust + WebAssembly プレイグラウンド

## 概要

このプロジェクトは、ルービックキューブの状態と操作を群論の観点から実装したライブラリです。

### 構成

- **cube-core**: コアライブラリ（状態表現・群演算）
- **cube-cli**: CLIツール（検証・実験用）
- **cube-wasm**: WebAssembly公開インターフェース

## セットアップ

```bash
# 依存関係のインストール
cargo build

# テスト実行
cargo test-all

# フォーマットとLint
cargo fmt
cargo clippy-all
```

## 使い方

### CLI

```bash
# ヘルプ表示
cargo run --bin cube-cli -- --help

# アルゴリズムの逆手を計算
cargo run --bin cube-cli -- inverse "R U R' U'"

# アルゴリズムの合成
cargo run --bin cube-cli -- compose "R U" "R' U'"

# べき乗
cargo run --bin cube-cli -- power "R U R' U'" 6
```

### WASM

```bash
# WASMビルド
cargo wasm-build

# WASMテスト
cargo wasm-test
```

## ライセンス

MIT
