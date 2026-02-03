use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

declare_id!("Eg9p74Ue3eMDiK86u3LzYg7GLqpBqHjCrrdvd4dkVuMv");

#[program]
pub mod bastion_p2p {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        amount: u64,
        fiat_reference: String,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.buyer = *ctx.accounts.buyer.key;
        escrow.seller = *ctx.accounts.seller.key;
        escrow.amount = amount;
        escrow.fiat_reference = fiat_reference;
        escrow.status = EscrowStatus::Locked;
        escrow.bump = ctx.bumps.escrow_vault;

        // Transfer USDC from buyer to the escrow vault
        let cpi_accounts = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.escrow_vault.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn release_escrow(ctx: Context<ReleaseEscrow>, _signature: String) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        
        require!(escrow.status == EscrowStatus::Locked, ErrorCode::InvalidStatus);
        
        escrow.status = EscrowStatus::Released;

        let seeds = &[
            b"escrow_vault",
            escrow.to_account_info().key.as_ref(),
            &[escrow.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.escrow_vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, escrow.amount)?;

        Ok(())
    }

    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        
        require!(escrow.status == EscrowStatus::Locked, ErrorCode::InvalidStatus);
        
        escrow.status = EscrowStatus::Cancelled;

        let seeds = &[
            b"escrow_vault",
            escrow.to_account_info().key.as_ref(),
            &[escrow.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.escrow_vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, escrow.amount)?;

        Ok(())
    }
}

#[account]
pub struct Escrow {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub fiat_reference: String,
    pub status: EscrowStatus,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscrowStatus {
    Locked,
    Released,
    Cancelled,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Escrow is not in a locked state.")]
    InvalidStatus,
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        init, 
        payer = buyer, 
        space = 8 + 32 + 32 + 8 + 128 + 8 + 1
    )]
    pub escrow: Account<'info, Escrow>,
    
    #[account(
        init,
        payer = buyer,
        seeds = [b"escrow_vault", escrow.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = escrow_vault,
    )]
    pub escrow_vault: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: The intended recipient
    pub seller: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    
    #[account(
        mut,
        seeds = [b"escrow_vault", escrow.key().as_ref()],
        bump = escrow.bump
    )]
    pub escrow_vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub witness: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    
    #[account(
        mut,
        seeds = [b"escrow_vault", escrow.key().as_ref()],
        bump = escrow.bump
    )]
    pub escrow_vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
