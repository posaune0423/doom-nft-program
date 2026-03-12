use anchor_lang::prelude::*;

use crate::{constants::GLOBAL_CONFIG_SEED, error::DoomNftProgramError, state::GlobalConfig};

#[derive(Accounts)]
pub struct TransferAdmin<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = admin @ DoomNftProgramError::Unauthorized,
        has_one = upgrade_authority @ DoomNftProgramError::Unauthorized
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub admin: Signer<'info>,
    pub upgrade_authority: Signer<'info>,
    pub new_admin: Signer<'info>,
}

pub fn process_transfer_admin(ctx: Context<TransferAdmin>) -> Result<()> {
    ctx.accounts.global_config.admin = ctx.accounts.new_admin.key();
    Ok(())
}
