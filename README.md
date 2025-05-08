# 🧾 governor-contract

A smart contract for on-chain governance. 
Written in Rust using [ink!](https://use.ink/) for Substrate-based blockchains.

---

## ✨ Features

- Submit and store hash-based proposals
- Vote with YES/NO (1 address = 1 vote)
- View results live or after voting ends
- Agent-compatible via MCP-governance module

---

## 🛠 Build & Test

```bash
cargo +nightly contract build
cargo +nightly test
```

> Requires `cargo-contract` and `substrate-contracts-node`.

---

## 📦 Project Structure

```
/src
  main.rs          # ink! smart contract
Cargo.toml         # Project config
README.md          # This file
```

---

## 🧠 Integration

This contract can be used by:

- Agents: to submit votes
- MCP modules: to fetch current results
- Wallet UIs: to display open proposals
