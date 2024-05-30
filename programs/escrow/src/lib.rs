use anchor_lang::prelude::*;

declare_id!("91aPi6eEmNW9GDfn9ypRM7ydLT4mhdLVh8kQ57xAv3tS");

mod state;

mod contexts;
use contexts::*;

#[program]
pub mod escrow {

    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_close()?;
        Ok(())
    }

    
}
