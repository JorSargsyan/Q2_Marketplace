#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
declare_id!("H4QHUFUkNQqrCcPQ45cwhPveXtr9mZ1723SZLNX4kvAs");

mod instructions;
mod state;

use crate::instructions::*;
use crate::state::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,name:String, fee:u16,) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>,price: u64) -> Result<()>{
        ctx.accounts.create_listing(price,&ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }

    pub fn puchase(ctx: Context<Purchase>)->Result<()>{
        
        Ok(())
    }
}
