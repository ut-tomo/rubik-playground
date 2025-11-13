#!/bin/bash
set -e

echo "🔨 Building WASM module..."
wasm-pack build --target web

echo "✅ Build complete!"
echo ""
echo "To run the playground:"
echo "  python3 -m http.server 8000"
echo "  or"
echo "  npx http-server -p 8000"
echo ""
echo "Then open: http://localhost:8000"
