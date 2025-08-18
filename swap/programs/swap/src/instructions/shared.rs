use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authorithy: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>
) -> Result<()> {
    let transfer_accounts_options = TransferChecked{
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authorithy.to_account_info(),
        mint: mint.to_account_info(),
    };

    // CPI is used to call the transfer_checked function from the token program
    // This function transfers tokens from one account to another, checking the amount and mint
    let cpi_context = CpiContext::new
        (token_program.to_account_info(),
        transfer_accounts_options);

    transfer_checked(cpi_context, *amount, mint.decimals);
    Ok(())
}
