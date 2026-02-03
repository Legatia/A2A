use anchor_lang::prelude::*;

declare_id!("BstnP2P111111111111111111111111111111111111");

#[program]
pub mod bastion_p2p {
    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>, amount: u64, fiat_details: String) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.buyer = *ctx.accounts.buyer.key;
        escrow.amount = amount;
        escrow.fiat_details = fiat_details;
        escrow.status = EscrowStatus::Locked;
        Ok(())
    }

    pub fn release_escrow(ctx: Context<ReleaseEscrow>, proof: String) -> Result<()> {
        // Verification logic for Bastion Proof-of-Transfer goes here
        let escrow = &mut ctx.accounts.escrow;
        escrow.status = EscrowStatus::Released;
        Ok(())
    }
}

#[account]
pub struct Escrow {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub fiat_details: String,
    pub status: EscrowStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscrowStatus {
    Locked,
    Released,
    Cancelled,
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(init, payer = buyer, space = 8 + 32 + 32 + 8 + 256 + 8)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    pub witness: Signer<'info>, // This will be the Bastion Proxy key
}
