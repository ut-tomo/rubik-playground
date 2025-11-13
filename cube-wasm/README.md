# Rubik's Cube WASM Playground

WebAssembly版のルービックキューブ3Dプレイグラウンド

## セットアップ

### 1. wasm-packのインストール

```bash
cargo install wasm-pack
```

### 2. WASMのビルド

```bash
cd cube-wasm
wasm-pack build --target web
```

### 3. ローカルサーバーの起動

Pythonを使う場合:
```bash
python3 -m http.server 8000
```

またはNode.jsのhttp-serverを使う場合:
```bash
npx http-server -p 8000
```

### 4. ブラウザで開く

```
http://localhost:8000
```

## 機能

### 基本操作
- **クイックムーブボタン**: U, D, L, R, F, B の各面を時計回り/反時計回り/180度回転
- **3D表示**: Three.jsによるインタラクティブな3D可視化
  - マウスドラッグで視点回転
  - スクロールでズーム

### アルゴリズム
- **アルゴリズム入力**: "R U R' U'" のような手順を入力して適用
- **逆手計算**: 入力したアルゴリズムの逆手を自動計算
- **リセット**: 完成状態に戻す

### 高度な操作
- **交換子 [A, B]**: 2つのアルゴリズムから A B A' B' を計算
- **共役 A B A'**: 2つのアルゴリズムから A B A' を計算

### 情報表示
- **完成判定**: キューブが完成状態かリアルタイム表示
- **合法性チェック**: 現在の状態が到達可能かチェック
- **パリティ情報**: コーナー・エッジのパリティ表示

## アーキテクチャ

- **Rust (cube-core)**: キューブのロジックとアルゴリズム
- **Rust (cube-wasm)**: wasm-bindgenによるJavaScriptバインディング
- **Three.js**: 3D可視化とレンダリング
- **WebAssembly**: 高速な計算とシームレスなJS連携
