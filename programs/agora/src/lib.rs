use anchor_lang::prelude::*;

declare_id!("4tox9gQK3UW8VoJdDJfZeW4Zi38RST6iZAiGk1e9tGBi");

#[program]
pub mod agora {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
