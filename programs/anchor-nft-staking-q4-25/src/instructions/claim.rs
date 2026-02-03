use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{
    errors::StakeError,
    state::{StakeConfig, UserAccount},
};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        mint::token_program = token_program,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump,
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        // require!(self.);

        let mint_account = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[b"config".as_ref(), &[self.config.bump]]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_account,
            signer_seeds,
        );

        mint_to(cpi_ctx, self.user_account.points as u64)?;

        self.user_account.points = 0;

        Ok(())
    }
}
