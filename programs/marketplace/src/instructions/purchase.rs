use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        seeds = [b"marketplace",marketplace.name.as_bytes()],
        bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        mut,
        seeds = [b"treasury",marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
  
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"listing",marketplace.key().as_ref(),maker_mint.key().as_ref()],
        bump,
        close = maker
    )]
    pub listing: Account<'info,Listing>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"rewards",marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::authority = marketplace,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>,
    pub token_program: Interface<'info, TokenInterface>
}


impl<'info> Purchase<'info> {
      pub fn withdraw_nft(&mut self) -> Result <()> {
        let cpi_program = self.token_program.to_account_info();
        let seeds = &[
            self.marketplace.key().as_ref(),
            self.maker_mint.key().as_ref(),
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);

        transfer_checked(cpi_ctx,1,self.maker_mint.decimals)
    }
}