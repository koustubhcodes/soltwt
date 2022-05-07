use anchor_lang::prelude::*;

declare_id!("5ArNY34VRyGLvcsoM67nCc6uzAcmcMgYJ1RuUXQPdQze");

#[program]
pub mod soltwt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}