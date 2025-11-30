# Rubik & Group Theory Playground

An interactive Rubik's Cube demo powered by WebAssembly + Three.js.

🌐 Demo: https://ut-tomo.github.io/rubik-playground/

## Overview

This playground combines Rust-based cube logic (via WebAssembly) with Three.js 3D visualization, allowing you to manipulate and observe the cube directly in your browser. The cube's state (permutation and orientation) is displayed on screen, making it easy to visually verify the effects of algorithms.

## Basic Operations

- Notation: Singmaster notation (e.g., `R`, `U`, `F`, `R'`, `U2`)
- Algorithm input: Enter moves separated by spaces (e.g., `R U R' U'`)
- Quick Moves buttons allow rapid application of basic moves (U/D/L/R/F/B and their inverses/doubles)

## Advanced Operations

- Commutator: `[A, B] = A B A^{-1} B^{-1}` — Used to create localized cycles
- Conjugate: `m_2^{m_1} = m_1^{-1} m_2 m_1` — Setup moves to relocate algorithms to different positions

In the Advanced section, enter algorithms A and B to generate and apply Commutator or Conjugate results.

## Tech Stack

- Rust (cube core logic)
- WebAssembly (wasm-pack)
- Three.js (frontend 3D rendering)

## License

MIT
