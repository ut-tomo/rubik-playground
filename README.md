# 🧊 Rubik's Cube Playground

Rust + WebAssemblyで動くインタラクティブなルービックキューブシミュレーター

🌐 **[デモページ](https://ut-tomo.github.io/rubik-playground/)**

# Rubik & Group Theory Playground

WebAssembly + Three.js で動作する、インタラクティブなルービックキューブのデモページです。

🌐 デモ: https://ut-tomo.github.io/rubik-playground/

## 概要

このプレイグラウンドは、Rust 実装のキューブロジック（WebAssembly 経由）と Three.js による 3D 可視化を組み合わせ、ブラウザ上でキューブを操作・観察できるデモです。キューブの状態（置換・向き）が画面上に表示され、アルゴリズムの効果を視覚的に確認できます。

## すぐに試す

1. ブラウザで GitHub Pages のデモにアクセスする: https://ut-tomo.github.io/rubik-playground/
2. ローカルで動かす場合（開発環境が整っている場合）:

```bash
cd cube-wasm
wasm-pack build --target web
python3 -m http.server 8000
# ブラウザで http://localhost:8000 を開く
```

## 基本操作（簡易）

- 使用する記法: シングマスター記法（例: `R`, `U`, `F`, `R'`, `U2`）
- アルゴリズム入力欄に半角スペースで区切って記述します（例: `R U R' U'`）。
- Quick Moves ボタンで基本手（U/D/L/R/F/B とその逆・2回）をすばやく適用できます。

## 高度な操作

- Commutator（交換子）: `[A, B] = A B A^{-1} B^{-1}` — 局所的なサイクルを作るために使います。
- Conjugate（共役）: `m_2^{m_1} = m_1^{-1} m_2 m_1` — アルゴリズムを別位置へ移動させるセットアップムーブ。

画面の Advanced 欄で A/B を入力して Commutator / Conjugate の結果を生成・適用できます。

## 技術スタック

- Rust（キューブのコアロジック）
- WebAssembly（wasm-pack）
- Three.js（フロントの 3D 表示）

## ライセンス

MIT
