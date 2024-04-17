use anchor_lang::prelude::*;

declare_id!("4wcTXvHy5d6vytHCcMiokMqYdj26QY2o5bB6M4gA5uQ4");

#[program]
pub mod storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, balance: u64) -> Result<()> {
        ctx.accounts.storage_account.balance = balance;
        msg!("Changed data to: {}!", balance);
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

#[account]
pub struct StorageData {
    balance: u64,
}
