use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Token, TokenAccount, Transfer};

declare_id!("Escrow1111111111111111111111111111111111111");



#[program]
pub mod solana_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.maker_token_account.to_account_info(),
            to: ctx.accounts.vault_account.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        // Transfer tokens from Taker to Maker
        let cpi_accounts_taker = Transfer {
            from: ctx.accounts.taker_token_account.to_account_info(),
            to: ctx.accounts.maker_receive_node.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };
        let cpi_ctx_taker = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_taker);
        token::transfer(cpi_ctx_taker, ctx.accounts.escrow_account.amount_expected)?;

        // Transfer tokens from Vault to Taker
        let seeds = &[b"escrow".as_ref(), &[ctx.accounts.escrow_account.bump]];
        let signer = &[&seeds[..]];
        let cpi_accounts_vault = Transfer {
            from: ctx.accounts.vault_account.to_account_info(),
            to: ctx.accounts.taker_receive_node.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_ctx_vault = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts_vault, signer);
        token::transfer(cpi_ctx_vault, ctx.accounts.vault_account.amount)?;

        Ok(())
    }
}

#[account]
pub struct EscrowAccount {
    pub maker: Pubkey,
    pub amount_expected: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut)]
    pub maker_token_account: Account<'info, TokenAccount>,
    #[account(init, payer = maker, space = 8 + 32 + 8 + 1)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    pub taker: Signer<'info>,
    #[account(mut)]
    pub taker_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_receive_node: Account<'info, TokenAccount>,
    #[account(mut)]
    pub maker_receive_node: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
