use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_program::program::invoke_signed;

declare_id!("57ny2iMQRgFfNuViGELqLhje3SFHW4GYCE6tbb3VgvUY");

// Hardcoded receiver address for SOL payment
const PAYMENT_RECEIVER: &str = "DxioT1JQNh9zatS8V3BZbA5uSV1gDR9qjKR7PYzA8tsL"; // Replace with your actual wallet address
const SOL_PAYMENT_AMOUNT: u64 = 10_000_000; // 0.01 SOL in lamports

#[program]
pub mod nft_transfer {
    use super::*;

    pub fn transfer_sol_and_mint_nft(ctx: Context<TransferSolAndMintNft>) -> Result<()> {
        // Step 1: Transfer SOL from user to hardcoded address
        let transfer_ix = system_program::Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.payment_receiver.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_ix,
        );
        
        system_program::transfer(cpi_ctx, SOL_PAYMENT_AMOUNT)?;
        
        // Step 2: Mint one NFT token to the user
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;
        
        msg!("SOL transferred and NFT minted successfully!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSolAndMintNft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    /// CHECK: This is the mint authority
    pub mint_authority: AccountInfo<'info>,
    
    /// CHECK: This is not dangerous, we're just using it as a payment receiver
    #[account(
        mut,
        address = PAYMENT_RECEIVER.parse::<Pubkey>().unwrap(),
    )]
    pub payment_receiver: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
