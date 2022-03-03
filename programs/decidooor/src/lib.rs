// Create transaction to deposit funds into treasury
// Mint governance token and transfer to depositor quadratic way

use anchor_lang::prelude::*;
// use solana_sdk::account_info::IntoAccountInfo;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer, MintTo};
use anchor_spl::associated_token::AssociatedToken;
use anchor_lang::solana_program::{program::invoke, system_instruction};
// use anchor_lang::{ToAccountInfo, Id};
use std::str::FromStr;

declare_id!("74UV6yKRkHzzkATgrGzPTUhhML6u48ST1BHxjUiScg52");

pub const TREASURY_ACCOUNT: &str = "ANPzsMRRUsAhCjgG7VQxAC1hYFJChofQ239H33J12Gnj";
pub const GOV_TOKEN: &str = "PHHJQry4FhrtTJMscDYo8S26ksmi7GwW8TmcKJkppcp";
pub const PROGRAM_ID: &str = "74UV6yKRkHzzkATgrGzPTUhhML6u48ST1BHxjUiScg52";


#[program]
pub mod phhdao {
    use super::*;

    pub fn handle_deposit(ctx: Context<HandleDeposit>, amount: u64, mint_bump: u8) -> ProgramResult {
        msg!("Transfer deposit to treasury");
        ctx.accounts.transfer_deposit(amount)?;
        
        // msg!("Mint governance token to user");
        // let cpi_accounts = token::MintTo {
        //     mint: ctx.accounts.mint.to_account_info(),
        //     to: ctx.accounts.payer.to_account_info(),
        //     authority: ToAccountInfo::to_account_info(&id()),
        // };
    
        // let cpi_program = ctx.accounts.spl_program.to_account_info();
        // let cpi_mint_tx = CpiContext::new(cpi_program, cpi_accounts);
        
        // token::mint_to(cpi_mint_tx, 1)?;

        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_token_acc.to_account_info(),
                    authority: ctx.accounts.mint.to_account_info(),
                },
                &[&[&[], &[mint_bump]]],
            ),
            1,
        )?;
    

        Ok(())
    }
}



//token transfer is a cpi to the token program

#[derive(Accounts)]
#[instruction(mint_bump: u8)]
pub struct HandleDeposit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Hardcoded treasury account
    #[account(mut, address = Pubkey::from_str(TREASURY_ACCOUNT).unwrap())]
    pub treasury_account: AccountInfo<'info>,
    // pub spl_program: Program<'info, Token>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [],
        bump,
        mint::decimals = 0,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>, //FIXME: Constrain this to GOV_TOKEN const?
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    #[account(init_if_needed, payer = payer, associated_token::mint = mint, associated_token::authority = payer)]
    pub user_token_acc: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
#[instruction(mint_bump: u8)]
pub struct MintGovToken<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [],
        bump,
        mint::decimals = 0,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>, //FIXME: Constrain this to GOV_TOKEN const?

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init_if_needed, payer = payer, associated_token::mint = mint, associated_token::authority = payer)]
    pub destination: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> HandleDeposit<'info> {
    fn transfer_deposit(&self, amount: u64) -> ProgramResult {
        invoke(
            &system_instruction::transfer(self.payer.key, self.treasury_account.key, amount),
            &[
                self.payer.to_account_info(),
                self.treasury_account.clone(),
                self.system_program.to_account_info(),
            ],
        )
    }

    

}