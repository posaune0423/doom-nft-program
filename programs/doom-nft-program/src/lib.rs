use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

declare_id!("AavECgzCbVhHeBGAfcUgT1tYEC4N4B96E8XtF9H1fMGt");

#[program]
pub mod doom_nft_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Creating NFT mint: {}", ctx.accounts.mint.key());
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        msg!("Minting NFT token to: {}", ctx.accounts.token_account.key());

        // Mint exactly 1 token (NFT)
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1, // NFTなので1個のみ
        )?;

        msg!("NFT token minted successfully!");
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
        msg!(
            "Transferring NFT from {} to {}",
            ctx.accounts.from.key(),
            ctx.accounts.to.key()
        );

        // Transfer exactly 1 token (NFT)
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.from_token_account.to_account_info(),
                    to: ctx.accounts.to_token_account.to_account_info(),
                    authority: ctx.accounts.from.to_account_info(),
                },
            ),
            1, // NFTなので1個
        )?;

        msg!("NFT transferred successfully!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 0, // NFTなので小数点以下なし
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: The mint authority
    pub mint_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(
        mut,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: The mint authority
    pub mint_authority: Signer<'info>,

    /// CHECK: The recipient of the NFT
    pub recipient: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = from,
    )]
    pub from_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = to,
    )]
    pub to_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub from: Signer<'info>,

    /// CHECK: The recipient of the NFT
    pub to: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}
