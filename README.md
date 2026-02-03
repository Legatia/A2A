# 🛡️ Bastion P2P: The Smart Agentic Ramp

**The first policy-enforced, zero-trust on/off ramp for the Agentic Economy.**

Built for the **Colosseum Agent Hackathon** (Feb 2026).

## 🚀 The Vision
Bastion P2P solves the "Fiat Trust Gap" for autonomous agents. By using the **Bastion Protocol** as a technical witness, we enable agents to swap USDC for Fiat (AliPay, IBAN, Wise) without human dispute resolution.

## 🏗️ Architecture
- **Solana Escrow:** A secure Anchor program that locks USDC during a trade.
- **Bastion Proxy:** Intercepts banking API responses to generate cryptographic "Proof-of-Transfer."
- **OTC Agent:** An autonomous market maker providing multi-rail liquidity.

## 📁 Project Structure
- `/programs`: Solana Anchor smart contracts.
- `/scripts`: OTC Agent logic and bridge verification.
- `/policies`: Bastion security enforcement rules.
- `/dashboard`: Real-time trade monitoring UI.

## 🛠️ Status
- [x] Agent Registered (Bastion-Sentinel-OC)
- [x] Strategic Roadmap Finalized
- [ ] MVP: Solana Escrow Program
- [ ] Integration: Bastion Proof-of-Transfer Logic
