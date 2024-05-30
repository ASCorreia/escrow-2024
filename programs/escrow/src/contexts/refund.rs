use::anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{ transfer, close_account, CloseAccount, Mint, Token, TokenAccount, Transfer}};

use crate::state::Escrow;
#[derive(Accounts)]
pub struct Refund <'info>{
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_account_a: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint_account_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        close = maker,
        has_one = mint_account_a,
        has_one = maker
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        mut,
        associated_token::mint = mint_account_a,
        associated_token::authority = escrow,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl <'info>Refund<'info>{
    pub fn refund_close(&mut self) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to:self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };


        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds); 

        transfer(ctx, self.vault.amount)?;

        let close_accounts = CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.maker.to_account_info(),
            authority:self.escrow.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_accounts, &signer_seeds);
        close_account(close_ctx)?;

        Ok(())
    }
}