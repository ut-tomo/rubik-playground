# Rubik's Cube Playground

ルービックキューブの群演算を探索するための Rust + WebAssembly プレイグラウンド

🌐 **[デモページを見る](https://ut-tomo.github.io/rubik-playground/)**

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
cargo test --workspace

# フォーマットとLint
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
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

### WebAssembly

```bash
# WASMビルド
cargo build --package cube-wasm --target wasm32-unknown-unknown --release

# または wasm-pack を使用
wasm-pack build crates/cube-wasm --target web
```

## 実装状況

### ✅ 実装済み
- [x] 基本的なキューブの状態表現（コーナー、エッジ）
- [x] 18種類の基本手（U, U', U2, D, D', D2, R, R', R2, L, L', L2, F, F', F2, B, B', B2）
- [x] 群演算（単位元、合成、逆元、べき乗）
- [x] アルゴリズムのパース・表記
- [x] CLIツール
- [x] WebAssemblyバインディング
- [x] GitHub Pagesデモサイト（JavaScript版）
- [x] CI/CD（fmt, clippy, test）

### 🚧 実装予定
- [ ] **WASM統合**: デモページを実際のRust実装で動作させる
- [ ] **キューブ操作の実装**: 手を実際にキューブに適用するロジック
- [ ] **状態可視化**: キューブの展開図表示
- [ ] **スクランブル生成**: ランダムな初期状態の生成
- [ ] **ソルバー**: IDA*やKociembaアルゴリズムによる解法探索
- [ ] **パターンデータベース**: コーナー・エッジのパターンDB
- [ ] **3D可視化**: Three.jsなどでの3D表示
- [ ] **アニメーション**: 手の実行を視覚的に表示

## 開発ロードマップ

### Phase 1: コア機能（完了）
基本的なデータ構造と群演算の実装

### Phase 2: WASM統合（進行中）
デモページでRustコードを実際に動かす

### Phase 3: キューブシミュレーション
手の適用と状態遷移の完全実装

### Phase 4: ソルバー実装
最適解探索アルゴリズムの実装

### Phase 5: UI/UX強化
3D可視化とインタラクティブな操作

## 貢献

Issues、Pull Requestsは大歓迎です！

## ライセンス

MIT
