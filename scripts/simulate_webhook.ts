import axios from 'axios';

async function simulateWebhook() {
    const WEBHOOK_URL = "http://localhost:3000/verify-transfer";
    
    const mockTransfer = {
        transactionId: "ST-998877",
        amount: 1000,
        reference: "ESCROW_ABC_123",
        bankSignature: "bank_hmac_proof_xyz"
    };

    console.log("📡 Simulating Bank Webhook...");
    try {
        const response = await axios.post(WEBHOOK_URL, mockTransfer);
        console.log("📥 Bastion Proxy Response:", response.data);
    } catch (e: any) {
        console.error("❌ Simulation Failed:", e.message);
    }
}

simulateWebhook();
