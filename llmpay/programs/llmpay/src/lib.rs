use anchor_lang::prelude::*;

declare_id!("GPvcuxPtkwYaLr5y8PRuTtMFpdGGwQ1crnFqfWpw82s6");

#[program]
pub mod llmpay {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
