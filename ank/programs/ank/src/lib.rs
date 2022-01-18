use anchor_lang::prelude::*;

declare_id!("DoGiWHsUGypgyxtofEdD3qUYxTTjxewr54eGUqtJQQ4");

#[program]
pub mod ank {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("anchor hello world");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
