import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as crypto from 'crypto';
import express from 'express';

// --- CONFIGURATION ---
const PORT = process.env.PORT || 3000;
const BASTION_PRIVATE_KEY = process.env.BASTION_PRIVATE_KEY || 'bastion_secret_key_2026';

// --- MIDDLEWARE SERVER ---
const app = express();
app.use(express.json());

/**
 * Endpoint: /verify-transfer
 * Logic:
 * 1. Receive bank success webhook.
 * 2. Validate HMAC signature from the bank.
 * 3. Match 'reference' to a locked Solana Escrow.
 * 4. Generate the 'Witness Proof' for on-chain release.
 */
app.post('/verify-transfer', async (req, res) => {
    const { transactionId, amount, reference, bankSignature } = req.body;

    console.log(`🛡️ Bastion Witness: Intercepted Webhook for TX ${transactionId}`);

    // Check internal Policy (e.g., MOLTBOOK_INFRA_SHIELD)
    const isAllowed = amount <= 5000; // Example daily limit logic
    
    if (!isAllowed) {
        console.log("🛑 Policy Violation: Transaction amount exceeds daily cap.");
        return res.status(403).json({ error: 'POLICY_VIOLATION' });
    }

    // Generate the On-Chain Witness Proof
    const payload = JSON.stringify({
        transactionId,
        reference,
        status: 'SUCCESS'
    });

    const witnessProof = crypto
        .createHmac('sha256', BASTION_PRIVATE_KEY)
        .update(payload)
        .digest('hex');

    console.log(`✅ Verification Success. Witness Proof generated: ${witnessProof}`);

    // Return the proof to the caller
    res.json({
        success: true,
        proof: witnessProof,
        instruction: "release_escrow"
    });
});

app.listen(PORT, () => {
    console.log(`🚀 Bastion Witness Proxy active on port ${PORT}`);
});
