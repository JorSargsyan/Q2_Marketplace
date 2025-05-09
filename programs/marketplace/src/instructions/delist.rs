use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer_checked,TransferChecked},
    token_interface::{Mint,TokenAccount,TokenInterface},
};
use crate::state::{Listing,Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]

pub struct DeList <'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        seeds =[b"marketplace",name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account()]
    pub maker_mint: InterfaceAccount<'info,Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_ata: InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(),maker_mint.key().as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info,Listing>,
    pub token_program : Interface<'info,TokenInterface>,
    pub system_program : Program<'info,System>,

}

impl <'info> DeList <'info> {
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
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);

        transfer_checked(cpi_ctx,1,self.maker_mint.decimals)
        
    }
}