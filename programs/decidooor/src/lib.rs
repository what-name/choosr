// Create transaction to deposit funds into treasury
// Mint governance token and transfer to depositor quadratic way

use anchor_spl::token::Mint;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_lang::solana_program::{program::invoke, system_instruction};
use std::str::FromStr;

declare_id!("74UV6yKRkHzzkATgrGzPTUhhML6u48ST1BHxjUiScg52");

pub const TREASURY_ACCOUNT: &str = "ANPzsMRRUsAhCjgG7VQxAC1hYFJChofQ239H33J12Gnj";

// let treasury_pubkey = "3t9wRrwK99uEKzexCHdzxCumUKjG7bMxngQqFESMTJwx";

#[program]
pub mod phhdao {
    use super::*;

    pub fn handle_deposit(ctx: Context<HandleDeposit>, amount: u64) -> ProgramResult {
        msg!("Transfer deposit to treasury");
        ctx.accounts.transfer_deposit(amount)?;

        // use anchor_spl::{
        //     token,
        //     associated_token::AssociatedToken,
        //     token::{Mint, MintTo, Token, TokenAccount, Transfer, Burn},
        // };
        
        Ok(())

        // let mint_gov: MintTo = MintTo {}
    }
}



//token transfer is a cpi to the token program

#[derive(Accounts)]
pub struct HandleDeposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: Hardcoded treasury account
    #[account(mut, address = Pubkey::from_str(TREASURY_ACCOUNT).unwrap())]
    pub treasury_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    // pub spl_program: Program<'info, Token>,
    // pub mint: Account<'info, Mint>,
    // pub user_token_acc: Account<'info, TokenAccount>,
}

impl<'info> HandleDeposit<'info> {
    fn transfer_deposit(&self, amount: u64) -> ProgramResult {
        invoke(
            &system_instruction::transfer(self.owner.key, self.treasury_account.key, amount),
            &[
                self.owner.to_account_info(),
                self.treasury_account.clone(),
                self.system_program.to_account_info(),
            ],
        )
        // .map_err(Into::into)
    }
}
