# 🛡️ Bastion A2A: Technical Architecture & Execution Plan (v2)

**Reframing:** From "P2P Ramp" to **"Institutional Kill-Switch & Policy Gateway for Agentic Commerce"**

## 1. System Architecture: The Modular Split
To satisfy Tier-1 institutional requirements (Stripe, Coinbase, Banks), the system is divided into two distinct, auto-syncing layers:

### A. Settlement Layer (Solana / Anchor)
- **Role:** Atomic settlement and capital custody.
- **Philosophy:** "Safe Money." It only moves capital when presented with a valid cryptographic proof from an authorized authority.
- **Key Artifact:** `programs/bastion-p2p`

### B. Policy Layer (Bastion Middleware)
- **Role:** **The Institutional Kill-Switch.** Policy enforcement, state verification, and attestation.
- **Philosophy:** "High-Performance Brakes." It validates the "Physical Truth" of off-chain transfers and ensures agents cannot spend outside of regulatory and corporate guardrails.
- **Key Artifact:** `scripts/witness_bridge.ts` & `policies/MOLTBOOK_INFRA_SHIELD`

---

## 2. Hardening the "Grip" (Feedback & VC Audit Integration)

### A. Authorized Witness Registry (On-Chain)
- **Config PDA:** We have implemented a `ProtocolConfig` account that stores the `bastion_authority` pubkey.
- **Enforcement:** The `release_escrow` instruction strictly enforces that only the Bastion Middleware can trigger funds.

### B. Policy-Linked Attestations (The "CEO Shield")
- **Metadata:** Every escrow stores a `policy_hash`.
- **Audit Trail:** This allows a CFO or Regulator to map an on-chain transaction back to the specific internal policy decision that authorized it, providing the "Why" behind the "Move."

### C. Asymmetric Proof-of-Settlement
- **Verification:** Moving from "Listening to webhooks" to "Attesting State." We use the Bastion Proxy to generate cryptographic proofs of fiat delivery, reducing dependency on fragile bank APIs.

---

## 3. Technical Components

### Component 1: The Solana Escrow Program (Anchor)
- **PDAs:** 
  - `EscrowAccount`: Stores trade metadata (buyer, seller, amount, fiat_ref, status, expiry).
  - `EscrowVault`: A Token PDA that holds the USDC collateral.
- **Instruction Set:**
  - `initialize_escrow`: Buyer locks USDC and defines the fiat reference string.
  - `release_escrow(witness_signature)`: Gated release. Funds transfer to the seller *only* if the Bastion Witness signs the state transition.
  - `cancel_escrow`: Anti-griefing. Allows recovery of funds after a timeout.

### Component 2: The Bastion "Technical Witness" Proxy
- **Verification Engine:** Intercepts cryptographically signed webhooks from traditional payment rails.
- **Verification:** Validates that the amount and reference in the fiat transfer match the on-chain escrow.
- **Security Logic:** Enforces the `INFRA_ISOLATION_v2` policy to ensure agents act within predefined financial bounds.

### Component 3: OTC Liquidity Agent (Market Maker)
- **Role:** Facilitates fiat liquidity.
- **Handshake:** Operates under Bastion's policy guardrails, ensuring that every fiat payout is verified before claiming the USDC collateral.

---

## 4. Execution Roadmap (Days 4-10)

| Phase | Task | Status |
|-------|------|--------|
| **Phase 1** | **Protocol Hardening:** Integrate `ProtocolConfig` and `policy_hash` into Devnet contract. | ✅ COMPLETE |
| **Phase 2** | **The Kill-Switch:** Implement the middleware logic to freeze settlement if a policy is violated. | 🏗️ IN PROGRESS |
| **Phase 3** | **Verification Bridge:** Finalize the TEE-ready witness signer for fiat attestations. | 📅 Day 5 |
| **Phase 4** | **Strategic Outreach:** Sales-Lead sub-agent engaging key partners (Vex, PayGuard). | 🏗️ IN PROGRESS |
| **Phase 5** | **The Demo:** End-to-end "Safe Sourcing" simulation: Policy Check -> Fiat Move -> Atomic Settlement. | 📅 Day 8-9 |
| **Phase 6** | **Submission:** Project lockdown and Colosseum leaderboard sync. | 📅 Day 10 |

---
**"We don't just move money; we move the truth between worlds."**
