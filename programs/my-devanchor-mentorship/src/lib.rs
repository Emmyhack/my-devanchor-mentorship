use anchor_lang::prelude::*;

declare_id!("93XNRZ55njk1GPtTWY9Jdm3ko1iqPcWN5QophZr9krTU");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // buy function
    pub fn buy(ctx: Context<Buy>, product_name: String, amount: u64) -> Result<()> {
        Ok(()) // Signals successful transaction
    }
    // sell function
    pub fn sell(ctx: Context<Sell>, product_name: String, amount: u64) -> Result<()> {
        Ok(()) // Signals successful transaction
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Buy {}

#[derive(Accounts)]
pub struct Sell {}
