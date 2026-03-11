use anchor_lang::prelude::*;

use crate::{
    constants::GLOBAL_CONFIG_SEED, error::DoomNftProgramError, events::UpgradeAuthorityUpdated,
    state::GlobalConfig,
};

#[derive(Accounts)]
pub struct SetUpgradeAuthority<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = upgrade_authority @ DoomNftProgramError::Unauthorized
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub upgrade_authority: Signer<'info>,
}

pub fn process_set_upgrade_authority(
    ctx: Context<SetUpgradeAuthority>,
    new_upgrade_authority: Pubkey,
) -> Result<()> {
    let global_config = &mut ctx.accounts.global_config;
    let old_upgrade_authority = global_config.upgrade_authority;
    global_config.upgrade_authority = new_upgrade_authority;

    emit!(UpgradeAuthorityUpdated {
        old_upgrade_authority,
        new_upgrade_authority,
    });

    Ok(())
}
