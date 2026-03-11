use anchor_lang::prelude::*;

use crate::{constants::GLOBAL_CONFIG_SEED, error::DoomNftProgramError, state::GlobalConfig};

#[derive(Accounts)]
pub struct TransferAdmin<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = admin @ DoomNftProgramError::Unauthorized
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub admin: Signer<'info>,
}

pub fn process_transfer_admin(ctx: Context<TransferAdmin>, new_admin: Pubkey) -> Result<()> {
    ctx.accounts.global_config.admin = new_admin;
    Ok(())
}
