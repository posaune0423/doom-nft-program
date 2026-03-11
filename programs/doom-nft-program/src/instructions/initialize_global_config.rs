use anchor_lang::prelude::*;

use crate::{
    constants::GLOBAL_CONFIG_SEED, state::GlobalConfig, utils::validate_base_metadata_url,
};

#[derive(Accounts)]
pub struct InitializeGlobalConfig<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + GlobalConfig::INIT_SPACE,
        seeds = [GLOBAL_CONFIG_SEED],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process_initialize_global_config(
    ctx: Context<InitializeGlobalConfig>,
    base_metadata_url: String,
    upgrade_authority: Pubkey,
) -> Result<()> {
    validate_base_metadata_url(&base_metadata_url)?;

    let global_config = &mut ctx.accounts.global_config;
    global_config.admin = ctx.accounts.admin.key();
    global_config.upgrade_authority = upgrade_authority;
    global_config.next_token_id = 1;
    global_config.mint_paused = false;
    global_config.base_metadata_url = base_metadata_url;
    global_config.collection = Pubkey::default();
    global_config.collection_update_authority = Pubkey::default();
    global_config.bump = ctx.bumps.global_config;

    Ok(())
}
