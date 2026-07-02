# Code Reviewer (Tauri + Candle LLM)

> **This was an attempt to build a desktop application with a local AI code reviewer. The attempt was unsuccessful.**

## What it was supposed to do

A Tauri desktop app that loads a local GGUF LLM (via Candle) and reviews code for bugs, returning a structured JSON with error locations and fixes.

## Why it failed

- The project could never be compiled because `index.crates.io` (Fastly CDN) was unreachable from the development environment — ICMP ping succeeded but TCP port 443 was blocked by the network firewall.
- Without being able to `cargo check` or `cargo build`, all Rust dependencies (candle-core, candle-transformers, tokenizers) could never be downloaded or verified.

## Project structure

```
src-tauri/
  Cargo.toml          # Tauri 2 + Candle 0.8 dependencies
  src/main.rs          # Tauri command: loads GGUF model, runs inference
  src/model_wrapper.rs # QuantizedModel trait for Candle
src/
  index.html           # Frontend UI with text input and results
```

## Tech stack

- **Backend:** Rust, Tauri 2, Candle 0.8 (quantized LLM inference), tokenizers
- **Frontend:** Vanilla HTML/CSS/JS
- **Model:** TinyLlama 1.1B Chat Q4_K_M (GGUF format)
