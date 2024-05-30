use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_account_a: Pubkey,
    pub mint_account_b: Pubkey,
    pub amount_a: u64,
    pub bump: u8,
}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 8 + 32 + 32 + 32 + 8 + 1;
}
