# Rubik's Cube Playground

ルービックキューブの群演算を探索するための Rust + WebAssembly プレイグラウンド

🌐 **[デモページを見る](https://ut-tomo.github.io/rubik-playground/)**

## 概要

このプロジェクトは、ルービックキューブの状態と操作を群論の観点から実装したライブラリです。
3x3x3キューブの状態を置換と向きで表現し、手の適用、アルゴリズム操作、合法性チェック、サイクル分解などの機能を提供します。

### 構成

- **cube-core**: コアライブラリ（状態表現・群演算・アルゴリズム操作）
- **cube-cli**: CLIツール（検証・実験用）
- **cube-wasm**: WebAssembly + 3D可視化プレイグラウンド
- **docs/**: GitHub Pagesデモサイト（JavaScript実装）

## クイックスタート

### ローカルで3Dプレイグラウンドを起動

```bash
# WASMをビルド
cd cube-wasm
wasm-pack build --target web

# ローカルサーバーを起動
python3 -m http.server 8000

# ブラウザで開く
# http://localhost:8000
```

または、ビルドスクリプトを使用:

```bash
cd cube-wasm
./build.sh
python3 -m http.server 8000
```

## セットアップ

```bash
# ワークスペースのビルド
cargo build

# テスト実行
cargo test

# CLIツールの実行
cargo run -p cube-cli
```

## アーキテクチャ

### cube-core の構成

```
cube-core/src/
├── lib.rs        # モジュールルート、公開API
├── state.rs      # Cube構造体、基本操作
├── moves.rs      # Move列挙型、手の適用ロジック
├── alg.rs        # アルゴリズム操作（逆手、交換子、共役）
├── legality.rs   # 合法性チェック、パリティ計算
└── cycles.rs     # サイクル分解
```

### データ構造

- **Cube**: キューブの状態
  - `corner_perm: [u8; 8]` - コーナーの置換（0-7）
  - `corner_ori: [u8; 8]` - コーナーの向き（0-2, mod 3）
  - `edge_perm: [u8; 12]` - エッジの置換（0-11）
  - `edge_ori: [u8; 12]` - エッジの向き（0-1, mod 2）

- **Move**: 18種類の基本手
  - U, U2, Up (上面: 時計回り, 180度, 反時計回り)
  - D, D2, Dp (下面)
  - L, L2, Lp (左面)
  - R, R2, Rp (右面)
  - F, F2, Fp (前面)
  - B, B2, Bp (後面)

## 使い方

### Rustライブラリとして使用

```rust
use cube_core::{Cube, Move, apply_alg, invert_alg, commutator, conjugate};

// 完成状態のキューブを作成
let mut cube = Cube::identity();

// 手を適用
cube.apply_move(Move::R);
cube.apply_move(Move::U);

// アルゴリズムを適用
let sexy_move = vec![Move::R, Move::U, Move::Rp, Move::Up];
apply_alg(&mut cube, &sexy_move);

// 逆手を計算して適用
let inverse = invert_alg(&sexy_move);
apply_alg(&mut cube, &inverse);
assert!(cube.is_solved());

// 交換子 [A, B] = A B A' B'
let a = vec![Move::R, Move::U];
let b = vec![Move::L, Move::D];
let comm = commutator(&a, &b);

// 共役 A B A'
let conj = conjugate(&a, &b);

// 合法性チェック
let legality = cube.legality();
println!("Is legal: {}", legality.is_legal);

// サイクル分解
let corner_cycles = cube_core::corner_cycles(&cube);
let edge_cycles = cube_core::edge_cycles(&cube);
```

### CLI

```bash
# CLIツールを実行
cargo run -p cube-cli
```

現在のCLIは基本的なデモ機能を提供しています。

## 実装状況

### ✅ 実装済み（cube-core）

#### 基本機能
- [x] **Cube構造体**: 8コーナー + 12エッジの状態表現（置換と向き）
- [x] **18種類の基本手**: U/U2/Up, D/D2/Dp, L/L2/Lp, R/R2/Rp, F/F2/Fp, B/B2/Bp
- [x] **手の適用**: `Cube::apply_move()` で各面の回転を実装
- [x] **完成判定**: `Cube::is_solved()` で完成状態かチェック

#### アルゴリズム操作（alg.rs）
- [x] **アルゴリズム適用**: `apply_alg()` で手順列を適用
- [x] **逆手計算**: `invert_alg()` でアルゴリズムの逆手順を生成
- [x] **手の反転**: `Move::inverse()` で各手の逆手を取得
- [x] **交換子**: `commutator(A, B)` で [A, B] = A B A' B' を計算
- [x] **共役**: `conjugate(A, B)` で A B A' を計算

#### 群論的機能
- [x] **合法性チェック**: `Cube::legality()` で到達可能な状態か判定
  - コーナーパリティ
  - エッジパリティ
  - エッジ反転の総和（mod 2）
  - コーナー回転の総和（mod 3）
- [x] **サイクル分解**: `corner_cycles()`, `edge_cycles()` で置換のサイクル構造を取得
- [x] **パリティ計算**: `permutation_parity()` で置換の偶奇を判定

#### 開発環境
- [x] **CLIツール**: 基本的な動作確認用
- [x] **テストスイート**: 統合テスト7件
- [x] **GitHub Pagesデモ**: JavaScript実装のデモサイト
- [x] **WASM 3Dプレイグラウンド**: Three.jsによるインタラクティブな3D可視化

### 🚧 実装予定

#### Phase 1: 基本機能の拡張
- [ ] **手の表記パース**: "R U R' U'" のような文字列からアルゴリズムを生成
  - Move::from_notation() の実装
  - アルゴリズム全体のパース
- [ ] **アルゴリズムの簡約化**: 連続する同じ手の統合（R R → R2, R R R → R'）
- [ ] **キューブの表示**: 展開図やステッカー配色の文字列表現
- [ ] **スクランブル生成**: ランダムな初期状態の生成

#### Phase 2: WASM統合
- [x] **cube-wasmクレート**: wasm-bindgenを使ったバインディング
- [x] **WASMビルド環境**: wasm-packの設定
- [x] **JavaScriptインターフェース**: Rust実装をブラウザから呼び出し
- [x] **3D可視化**: Three.jsによるインタラクティブなキューブ表示
- [ ] **アニメーション**: 手の実行を滑らかに表示
- [ ] **GitHub Pagesへの統合**: WASMプレイグラウンドの公開デプロイ

#### Phase 3: 高度な機能
- [ ] **G1/G2/G3分析**: サブグループの判定
- [ ] **HTM/QTM距離**: 最短手数の推定
- [ ] **コーナー/エッジの分離ソルバー**: 部分的な解法探索
- [ ] **パターンデータベース**: コーナー3-cycle, エッジ3-cycleのDB

#### Phase 4: ソルバー実装
- [ ] **IDA*探索**: 基本的な最適解探索
- [ ] **Kociembaアルゴリズム**: 2段階解法
- [ ] **ヒューリスティック関数**: 効率的な探索のための推定関数
- [ ] **パターンデータベースとの統合**: DBを使った高速探索

#### Phase 5: UI/UX
- [ ] **3D可視化**: Three.jsでの3次元表示
- [ ] **アニメーション**: 手の実行を滑らかに表示
- [ ] **インタラクティブ操作**: マウス/タッチでキューブを回転
- [ ] **アルゴリズムエディタ**: 手順の入力・編集・実行

## 開発ロードマップ

### ✅ Phase 1: コア機能（完了）
基本的なデータ構造と群演算の実装
- Cube状態表現
- 18種類の基本手
- アルゴリズム操作（逆手、交換子、共役）
- 合法性チェックとサイクル分解

### ✅ Phase 2: WASM統合（完了）
デモページでRustコードを実際に動かす
- wasm-packの設定
- JavaScriptバインディング
- Three.jsによる3D可視化

### 🔄 Phase 3: 使いやすさの向上（次のステップ）
- 手の表記パース（"R U R' U'" → アルゴリズム）
- キューブの視覚的表現
- アルゴリズムの簡約化
- スクランブル生成
- アニメーション機能

### 🎯 Phase 4: 高度な機能
- G1/G2/G3分析とサブグループ判定
- HTM/QTM距離の推定
- パターンデータベース

### 🚀 Phase 5: ソルバー実装
最適解探索アルゴリズムの実装
- IDA*基本実装
- パターンデータベース
- Kociembaアルゴリズム

### 🎨 Phase 6: UI/UX強化
さらなる可視化とインタラクティブ性の向上
- アニメーション
- インタラクティブ編集
- GitHub Pagesへの統合デプロイ

## 技術スタック

- **言語**: Rust 2021 Edition
- **ビルドシステム**: Cargo workspace
- **テスト**: cargo test
- **デプロイ**: GitHub Pages
- **予定**: WebAssembly (wasm-pack)

## 参考資料

- [Rubik's Cube群論](https://en.wikipedia.org/wiki/Rubik%27s_Cube_group)
- [Kociemba's Algorithm](http://kociemba.org/cube.htm)
- [cubing.js](https://github.com/cubing/cubing.js) - JavaScriptキューブライブラリ

## 貢献

Issues、Pull Requestsは大歓迎です！

## ライセンス

MIT
