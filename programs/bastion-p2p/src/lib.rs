use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

declare_id!("Eg9p74Ue3eMDiK86u3LzYg7GLqpBqHjCrrdvd4dkVuMv");

#[program]
pub mod bastion_p2p {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        id: u64,
        amount: u64,
        fiat_reference: String,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.id = id;
        escrow.buyer = *ctx.accounts.buyer.key;
        escrow.seller = *ctx.accounts.seller.key;
        escrow.amount = amount;
        escrow.fiat_reference = fiat_reference;
        escrow.status = EscrowStatus::Locked;
        escrow.escrow_bump = ctx.bumps.escrow;
        escrow.vault_bump = ctx.bumps.escrow_vault;

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
        require!(ctx.accounts.escrow.status == EscrowStatus::Locked, ErrorCode::InvalidStatus);

        let escrow_seeds = &[
            b"escrow".as_ref(),
            ctx.accounts.escrow.buyer.as_ref(),
            &ctx.accounts.escrow.id.to_le_bytes(),
            &[ctx.accounts.escrow.escrow_bump],
        ];
        let signer = &[&escrow_seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, ctx.accounts.escrow.amount)?;

        ctx.accounts.escrow.status = EscrowStatus::Released;

        Ok(())
    }

    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        require!(ctx.accounts.escrow.status == EscrowStatus::Locked, ErrorCode::InvalidStatus);

        let escrow_seeds = &[
            b"escrow".as_ref(),
            ctx.accounts.escrow.buyer.as_ref(),
            &ctx.accounts.escrow.id.to_le_bytes(),
            &[ctx.accounts.escrow.escrow_bump],
        ];
        let signer = &[&escrow_seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, ctx.accounts.escrow.amount)?;

        ctx.accounts.escrow.status = EscrowStatus::Cancelled;

        Ok(())
    }
}

#[account]
pub struct Escrow {
    pub id: u64,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub fiat_reference: String,
    pub status: EscrowStatus,
    pub escrow_bump: u8,
    pub vault_bump: u8,
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
#[instruction(id: u64)]
pub struct InitializeEscrow<'info> {
    #[account(
        init,
        payer = buyer,
        space = 8 + 8 + 32 + 32 + 8 + 4 + 128 + 1 + 1 + 1,
        seeds = [b"escrow", buyer.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = buyer,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = escrow
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
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.buyer.as_ref(), &escrow.id.to_le_bytes()],
        bump = escrow.escrow_bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump = escrow.vault_bump
    )]
    pub escrow_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,

    pub witness: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.buyer.as_ref(), &escrow.id.to_le_bytes()],
        bump = escrow.escrow_bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump = escrow.vault_bump
    )]
    pub escrow_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,

    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
