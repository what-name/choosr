// Create transaction to deposit funds into treasury
// Mint governance token and transfer to depositor quadratic way

use anchor_lang::prelude::*;
// use solana_sdk::account_info::IntoAccountInfo;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer, MintTo};
use anchor_lang::solana_program::{program::invoke, system_instruction};
use anchor_lang::{ToAccountInfo, Id};
use std::str::FromStr;

declare_id!("74UV6yKRkHzzkATgrGzPTUhhML6u48ST1BHxjUiScg52");

pub const TREASURY_ACCOUNT: &str = "ANPzsMRRUsAhCjgG7VQxAC1hYFJChofQ239H33J12Gnj";
pub const GOV_TOKEN: &str = "PHHi5AsrPgWs32VtiB4PMGiudaFzjGC8Una5BEzvwLZ";
pub const PROGRAM_ID: &str = "74UV6yKRkHzzkATgrGzPTUhhML6u48ST1BHxjUiScg52";

// let treasury_pubkey = "3t9wRrwK99uEKzexCHdzxCumUKjG7bMxngQqFESMTJwx";

#[program]
pub mod phhdao {
    use super::*;

    pub fn handle_deposit(ctx: Context<HandleDeposit>, amount: u64) -> ProgramResult {
        msg!("Transfer deposit to treasury");
        ctx.accounts.transfer_deposit(amount)?;
        
        msg!("Mint governance token to user");
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
            authority: ToAccountInfo::to_account_info(&id()),
        };
    
        let cpi_program = ctx.accounts.spl_program.to_account_info();
        let cpi_mint_tx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::mint_to(cpi_mint_tx, 1)?;

        Ok(())
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
    pub spl_program: Program<'info, Token>,
    pub mint: Account<'info, Mint>,
    pub user_token_acc: Account<'info, TokenAccount>,
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

    // fn mint_gov_otken(&self, amount: u64) -> ProgramResult {
    //     // invoke(&anchor_spl::token::mint_to(self.mint, amount), account_infos: &[AccountInfo])
    //     invoke(
    //         instruction: &Instruction, //mintTo instruction
    //         token::MintTo {
                
    //         } //1 mint authority (program), 2. user's address (to), 3.mint pubkey
            
    //     )
    // }

    // fn mint_gov_token(&self, amount: u64) -> ProgramResult {

    // }
}


// MINT EXAMPLE


// pub fn exchange(ctx: Context<Exchange>, amount_to_burn: u64, amount_to_give: u64) -> ProgramResult {

//     let cpi_accounts = token::Burn {
//         mint: ctx.accounts.burn_mint.to_account_info(),
//         to: ctx.accounts.burn_token_acct.to_account_info(),
//         authority: ctx.accounts.token_acct_owner.to_account_info(),
//     };
//     let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
//     token::burn(cpi_ctx, amount_to_burn)?;

//     // token burnt at this point

//     msg!("Token burnt, time to give them the new token");

//     if ctx.accounts.mint_authority.key() != ctx.accounts.receive_mint.mint_authority.unwrap() {
//         msg!("error");
//         msg!("ctx.accounts.mint_authority: {:?}", ctx.accounts.mint_authority.key());
//         msg!("ctx.accounts.receive_mint.mint_authority: {:?}", ctx.accounts.receive_mint.mint_authority);
//         return Err(ErrorCode::InvalidOwner.into());

//     let cpi_accounts = token::MintTo {
//         mint: ctx.accounts.receive_mint.to_account_info(),
//         to: ctx.accounts.to_receive_token_acct.to_account_info(),
//         authority: ctx.accounts.mint_authority.to_account_info()
//     };

//     let cpi_program = ctx.accounts.token_program.to_account_info();
//     let cpi_mint_tx = CpiContext::new(cpi_program, cpi_accounts);
    
//     token::mint_to(cpi_mint_tx, 1)?;

//     Ok(())
// }

// pub fn initialize_mint(ctx: Context<InitializeMint>) -> ProgramResult {

//     let mint_accounts = SetAuthority {
//         current_authority: ctx.accounts.mint_authority.to_account_info().clone(),
//         account_or_mint: ctx.accounts.mint.to_account_info().clone(),
//     };
//     let cpi_program = ctx.accounts.token_program.to_account_info();
//     let (pda, _authority_bump) =
//         Pubkey::find_program_address(&[PDA_SEED], ctx.program_id);

//     token::set_authority(
//         CpiContext::new(cpi_program, mint_accounts),
//         AuthorityType::MintTokens,
//         Some(pda),
//     )?;

//     Ok(())
// }