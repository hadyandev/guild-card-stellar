# 🏴‍☠️ Guild Card - Stellar Soulbound Token

> *"Where brave souls find their place among legends"*

A Soulbound Token (SBT) smart contract built on Stellar blockchain using Soroban — inspired by RPG guild membership cards where achievements and identity are permanently bound to their owner.

## 🎯 Project Overview

Guild Card is a **non-transferable membership token** (Soulbound Token) that represents exclusive membership in a guild or organization on the Stellar blockchain.

### Key Features

| Feature | Description |
|---------|-------------|
| 🎫 **Soulbound Membership** | Non-transferable token bound to wallet address |
| 🏆 **Rank System** | Auto-leveling based on experience points (C → B → A → S rank) |
| 📈 **EXP Accumulation** | Earn experience through activities |
| 🔒 **On-chain Identity** | Immutable membership record stored on Stellar |

## 📝 Use Cases

1. **Community Memberships** — Exclusive access to DAOs, clubs, or gaming guilds
2. **Achievement Badges** — Proof of participation/attendance that cannot be sold
3. **Certifications** — Educational credentials or workshop completions
4. **Gaming Guilds** — RPG-style rank progression (inspired by anime guild systems)

## 🛠️ Technology Stack

- **Blockchain**: Stellar (Soroban smart contracts)
- **Language**: Rust
- **SDK**: Soroban SDK
- **Network**: Testnet

## 🚀 Quick Start

### Prerequisites

- Rust installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Stellar CLI: `cargo install --locked stellar-cli`
- Testnet funded wallet

### Build & Deploy

```bash
# 1. Build WASM
cd contracts/guild_card
cargo build --target wasm32v1-none --release

# 2. Generate testnet wallet
stellar keys generate guild-card --network testnet --fund

# 3. Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32v1-none/release/guild_card.wasm \
  --source-account guild-card \
  --network testnet
```

### Interact with Contract

```bash
# Mint your guild membership card
stellar contract invoke \
  --id CONTRACT_ID \
  --source-account guild-card \
  --network testnet \
  -- \
  mint \
  --owner YOUR_ADDRESS \
  --guild "Straw Hats" \
  --member_name "your_name"

# Check your guild card
stellar contract invoke \
  --id CONTRACT_ID \
  --source-account guild-card \
  --network testnet \
  -- \
  get \
  --owner YOUR_ADDRESS

# Add experience points (level up!)
stellar contract invoke \
  --id CONTRACT_ID \
  --source-account guild-card \
  --network testnet \
  -- \
  add_exp \
  --owner YOUR_ADDRESS \
  --amount 100

# Check your current rank
stellar contract invoke \
  --id CONTRACT_ID \
  --source-account guild-card \
  --network testnet \
  -- \
  get_rank \
  --owner YOUR_ADDRESS
```

## 📊 Smart Contract Details

**Testnet Contract ID**:
```
C... (add after deploy)
```

**Contract Functions**:
| Function | Description |
|----------|-------------|
| `mint(owner, guild, member_name)` | Mint new soulbound membership card |
| `get(owner)` | View full membership card details |
| `add_exp(owner, amount)` | Add experience points (auto rank up) |
| `get_exp(owner)` | Get current experience points |
| `get_rank(owner)` | Get current rank (C/B/A/S) |

### Rank Progression

| Rank | Required EXP | Description |
|------|--------------|-------------|
| C | 0-999 | Novice - Starting rank |
| B | 1,000-4,999 | Adept - Rising member |
| A | 5,000-9,999 | Master - Seasoned member |
| S | 10,000+ | Legend - Elite status |

## 📁 Project Structure

```
stellar-sbt/
├── README.md                # This file
├── Cargo.toml               # Workspace config
└── contracts/
    └── guild_card/
        ├── Cargo.toml
        └── src/
            └── lib.rs       # Smart contract code
```

## 🎭 Anime Inspiration

This project draws inspiration from guild systems in anime like **One Piece** (Straw Hats), **Fairy Tail**, and **Solo Leveling** — where membership is a badge of honor, not a tradable commodity.

## 🙏 Credits

Built for **Rise In x Stellar Workshop**

Developer: [@hadyandev](https://github.com/hadyandev)

---

**Q.E.D. 💠**