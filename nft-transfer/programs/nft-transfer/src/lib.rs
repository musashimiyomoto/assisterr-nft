use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};

declare_id!("Gs7vEJ2MRS8GF35Us9CX19PE8dowJU7gsXTbqCFN36Vb");

// Hardcoded receiver address for SOL payment
const PAYMENT_RECEIVER: &str = "DxioT1JQNh9zatS8V3BZbA5uSV1gDR9qjKR7PYzA8tsL"; // Replace with your actual wallet address

// NFT tier prices
const TIER1_PRICE: u64 = 1_000_000_000; // 1 SOL in lamports
const TIER2_PRICE: u64 = 500_000_000;    // 0.5 SOL in lamports
const TIER3_PRICE: u64 = 100_000_000;    // 0.1 SOL in lamports

#[program]
pub mod nft_transfer {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, payment_amount: u64) -> Result<()> {
        // Validate payment amount and determine the NFT tier
        let tier = if payment_amount >= TIER1_PRICE {
            "Tier 1"
        } else if payment_amount >= TIER2_PRICE {
            "Tier 2"
        } else if payment_amount >= TIER3_PRICE {
            "Tier 3"
        } else {
            return Err(error!(ErrorCode::InsufficientPayment));
        };
        
        // Step 1: Transfer SOL from user to hardcoded address
        let transfer_ix = system_program::Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.payment_receiver.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_ix,
        );
        
        system_program::transfer(cpi_ctx, payment_amount)?;
        
        // Step 2: Mint one NFT token to the user's token account
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;
        
        msg!("NFT {} created and minted successfully!", tier);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(payment_amount: u64)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user,
        mint::decimals = 0,
        mint::authority = user.key(),
        mint::freeze_authority = user.key(),
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        token::mint = mint,
        token::authority = user,
    )]
    pub token_account: Account<'info, TokenAccount>,
    
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

#[error_code]
pub enum ErrorCode {
    #[msg("Payment amount is too low")]
    InsufficientPayment,
}
