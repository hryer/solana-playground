use anchor_lang::prelude::*;
use anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    };

use crate::{Offer, ANCHOR_DISCRIMINATOR};

use super::transfer_tokens;

#[derive(Accounts)]
pub struct TakeOffer<'info>{
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,

    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,
}

// 5:20:13 YTB SOL BOOTCAMP
