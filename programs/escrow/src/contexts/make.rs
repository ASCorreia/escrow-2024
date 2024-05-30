use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount, Transfer, transfer}};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make <'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_account_a: Account<'info, Mint>,
    pub mint_account_b: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        seeds = [b"escrow", user.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Escrow::INIT_SPACE)
        ]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_account_a,
        associated_token::authority = user,

    )]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint_account_a,
        associated_token::authority = escrow,
    )]
    pub maker_vault: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl <'info>Make<'info>{
    pub fn init(&mut self, seed:u64, amount_a:u64, bumps:&MakeBumps) -> Result<()>{
        self.escrow.set_inner(Escrow{
            seed,
            maker: self.user.key(),
            mint_account_a: self.mint_account_a.key(),
            mint_account_b: self.mint_account_b.key(),
            amount_a,
            bump: bumps.escrow,
        });
        Ok(())
    }

    pub fn deposit(&mut self, amount:u64) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.maker_ata.to_account_info(),
            to: self.maker_vault.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}
