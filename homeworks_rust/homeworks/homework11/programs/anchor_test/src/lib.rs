use anchor_lang::prelude::*;

declare_id!("89m59ppH36q9YUJYzwKGRWLGePtYU5wbHt8mKq3VKhHg");

#[program]
pub mod anchor_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.balance = 100;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        if vault.balance + amount > 1000 {
            return Err(ErrorCode::VaultBalanceExceeds1000.into());
        }
        vault.balance += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}

#[account]
pub struct Vault {
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Vault balance exceeds 1000")]
    VaultBalanceExceeds1000,
}
