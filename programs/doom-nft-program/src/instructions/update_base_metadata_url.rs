use anchor_lang::prelude::*;

use crate::{
    constants::GLOBAL_CONFIG_SEED, error::DoomNftProgramError, events::BaseMetadataUrlUpdated,
    state::GlobalConfig, utils::validate_base_metadata_url,
};

#[derive(Accounts)]
pub struct UpdateBaseMetadataUrl<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = admin @ DoomNftProgramError::Unauthorized
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub admin: Signer<'info>,
}

pub fn process_update_base_metadata_url(
    ctx: Context<UpdateBaseMetadataUrl>,
    base_metadata_url: String,
) -> Result<()> {
    validate_base_metadata_url(&base_metadata_url)?;

    let global_config = &mut ctx.accounts.global_config;
    let old_base_url = global_config.base_metadata_url.clone();
    global_config.base_metadata_url = base_metadata_url.clone();

    emit!(BaseMetadataUrlUpdated {
        old_base_url,
        new_base_url: base_metadata_url,
    });

    Ok(())
}
