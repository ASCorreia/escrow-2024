use::anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{ transfer,close_account, CloseAccount, Mint, Token, TokenAccount, Transfer},};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take <'info>{
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_account_a: Box<Account<'info, Mint>>,
    pub mint_account_b: Box<Account<'info, Mint>>,
    #[account(
        mut,
        has_one = mint_account_a,
        has_one = mint_account_b,
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        close = maker
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        mut,
        associated_token::mint = mint_account_a,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_account_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_account_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_account_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl <'info>Take<'info>{
    pub fn deposit(&mut self) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_context, self.escrow.amount_a)?;

        Ok(())
    }

    pub fn withdraw_close(&mut self) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        transfer(cpi_context, self.escrow.amount_a)?;

        let close_accounts = CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.taker.to_account_info(),
            authority:self.escrow.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_accounts, &signer_seeds);
        close_account(close_ctx)?;
        Ok(())
    }
}
