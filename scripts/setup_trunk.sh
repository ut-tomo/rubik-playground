#!/bin/bash
# trunk（WASMビルドツール）のセットアップスクリプト

set -e

echo "=== Trunk セットアップ ==="

# trunkのインストール確認
if ! command -v trunk &> /dev/null; then
    echo "trunk をインストール中..."
    cargo install --locked trunk
else
    echo "✓ trunk は既にインストールされています"
fi

# wasm-bindgen-cliのインストール確認
if ! command -v wasm-bindgen &> /dev/null; then
    echo "wasm-bindgen-cli をインストール中..."
    cargo install --locked wasm-bindgen-cli
else
    echo "✓ wasm-bindgen-cli は既にインストールされています"
fi

# WASMターゲットの追加
echo "WASM ターゲットを追加中..."
rustup target add wasm32-unknown-unknown

echo ""
echo "✓ セットアップ完了！"
echo ""
echo "使い方:"
echo "  trunk serve web/index.html    # 開発サーバー起動"
echo "  trunk build web/index.html    # 本番ビルド"
