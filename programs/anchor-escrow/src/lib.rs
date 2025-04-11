#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("23jYQm51u9LmG5ifKPcDnvUCfyPT1SdSJXoHYRS32teU");

#[program]
pub mod anchor_escrow {
    use super::*;

    // make
    // - init escrow , deposit
    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit);
        Ok(())
    }

    // refund
    // - deposit , close
    // take
    // - deposit, withdraw, close vault
}

#[derive(Accounts)]
pub struct Initialize {}
