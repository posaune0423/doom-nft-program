use anchor_lang::prelude::*;

use crate::{constants::GLOBAL_CONFIG_SEED, error::DoomNftProgramError, state::GlobalConfig};

#[derive(Accounts)]
pub struct SetMintPaused<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = admin @ DoomNftProgramError::Unauthorized
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub admin: Signer<'info>,
}

pub fn process_set_mint_paused(ctx: Context<SetMintPaused>, paused: bool) -> Result<()> {
    ctx.accounts.global_config.mint_paused = paused;
    Ok(())
}
