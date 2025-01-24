use anchor_lang::prelude::*;

declare_id!("4DKsyTYThSeZgoYP7We2Y8gmkfQAxobShwJmz1brgsGz");

#[program]
pub mod my_devanchor_mentorship {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
