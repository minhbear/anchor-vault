use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};


declare_id!("J6Fc9DF2SxEydxG7BZhB7CZybPxSTnfo27r6Z8RyVuJb");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<Vault>, amount: u64) -> Result<()> {
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)
    }

    pub fn withdraw(ctx: Context<Vault>, amount: u64) -> Result<()> {
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };

        let seeds = &[
            "vault".as_bytes(),
            ctx.accounts.owner.key.as_ref(),
            &[ctx.bumps.vault],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Vault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}