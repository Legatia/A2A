import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BastionP2p } from "../target/types/bastion_p2p";
import { 
  TOKEN_PROGRAM_ID, 
  createMint, 
  createAccount, 
  mintTo, 
  getAccount 
} from "@solana/spl-token";
import { assert } from "chai";

describe("bastion-p2p", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BastionP2p as Program<BastionP2p>;
  
  let mint: anchor.web3.Pubkey;
  let buyerTokenAccount: anchor.web3.Pubkey;
  let sellerTokenAccount: anchor.web3.Pubkey;
  let escrowVault: anchor.web3.Pubkey;
  
  const buyer = anchor.web3.Keypair.generate();
  const seller = anchor.web3.Keypair.generate();
  const witness = anchor.web3.Keypair.generate(); // Bastion Witness
  const escrow = anchor.web3.Keypair.generate();

  const amount = new anchor.BN(1000);
  const fiatReference = "TXN_12345";

  before(async () => {
    // Airdrop SOL to buyer
    const signature = await provider.connection.requestAirdrop(buyer.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature);

    // Create USDC mint
    mint = await createMint(
      provider.connection,
      buyer,
      buyer.publicKey,
      null,
      6
    );

    // Create token accounts
    buyerTokenAccount = await createAccount(provider.connection, buyer, mint, buyer.publicKey);
    sellerTokenAccount = await createAccount(provider.connection, buyer, mint, seller.publicKey);

    // Mint tokens to buyer
    await mintTo(
      provider.connection,
      buyer,
      mint,
      buyerTokenAccount,
      buyer.publicKey,
      2000
    );

    // Derive PDA for escrow vault
    [escrowVault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow_vault"), escrow.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initializes the escrow!", async () => {
    await program.methods
      .initializeEscrow(amount, fiatReference)
      .accounts({
        escrow: escrow.publicKey,
        escrowVault: escrowVault,
        mint: mint,
        buyer: buyer.publicKey,
        seller: seller.publicKey,
        buyerTokenAccount: buyerTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      } as any)
      .signers([buyer, escrow])
      .rpc();

    const escrowAccount = await program.account.escrow.fetch(escrow.publicKey);
    assert.equal(escrowAccount.buyer.toBase58(), buyer.publicKey.toBase58());
    assert.equal(escrowAccount.amount.toNumber(), 1000);
    assert.equal(escrowAccount.fiatReference, fiatReference);
    
    const vaultAccount = await getAccount(provider.connection, escrowVault);
    assert.equal(vaultAccount.amount.toString(), "1000");
  });

  it("Releases the escrow via witness!", async () => {
    // Airdrop SOL to witness
    const signature = await provider.connection.requestAirdrop(witness.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature);

    await program.methods
      .releaseEscrow("MOCK_SIGNATURE")
      .accounts({
        escrow: escrow.publicKey,
        escrowVault: escrowVault,
        sellerTokenAccount: sellerTokenAccount,
        witness: witness.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      } as any)
      .signers([witness])
      .rpc();

    const sellerAccount = await getAccount(provider.connection, sellerTokenAccount);
    assert.equal(sellerAccount.amount.toString(), "1000");
    
    const escrowAccount = await program.account.escrow.fetch(escrow.publicKey);
    assert.deepEqual(escrowAccount.status, { released: {} });
  });
});
