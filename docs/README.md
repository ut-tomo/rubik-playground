# Rubik & Group Theory Playground (docs)

この `docs/` フォルダは GitHub Pages に配置するデプロイ用コンテンツ用です。内容はトップの README と同様に、デモページの簡潔な説明を掲載しています。

🌐 デモ: https://ut-tomo.github.io/rubik-playground/

## 概要

ブラウザ上で Rust 実装（WebAssembly）と Three.js による 3D 可視化を組み合わせたインタラクティブなルービックキューブのデモです。UI から基本手やアルゴリズムを入力して動作を確認できます。


## 操作のポイント

- シングマスター記法で手順を入力（例: `R U R' U'`）。
- Quick Moves、Algorithm、Advanced（Commutator / Conjugate）を UI 上で使用できます。

## 技術スタック

- Rust, wasm-pack, WebAssembly
- Three.js

## ライセンス

MIT
