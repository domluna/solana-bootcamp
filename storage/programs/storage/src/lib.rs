use anchor_lang::prelude::*;

declare_id!("4wcTXvHy5d6vytHCcMiokMqYdj26QY2o5bB6M4gA5uQ4");

#[program]
pub mod storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let balance = 100;
        ctx.accounts.storage_account.balance = balance;
        msg!("Initialized data to: {}!", balance);
        Ok(())
    }

    pub fn update_balance(ctx: Context<UpdateBalance>) -> Result<()> {
        let data: &mut Account<StorageData> = &mut ctx.accounts.storage_account;          
        if data.balance >= 1000 {
            return Err(ErrorCode::BalanceLimitReached.into());
        }
        data.balance += 100;
        msg!("Changed data to: {}!", data.balance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds=[signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + 8
    )]
    pub storage_account: Account<'info, StorageData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBalance<'info> {
    #[account(mut, 
        constraint = storage_account.balance > 0,
    )]       
    pub storage_account: Account<'info, StorageData>,
}

#[account]
pub struct StorageData {
    balance: u64,
}


#[error_code]
pub enum ErrorCode {
    #[msg("Balance limit of 1000 reached or exceeded.")]
    BalanceLimitReached,
}
