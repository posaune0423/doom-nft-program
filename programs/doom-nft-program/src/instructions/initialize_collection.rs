use anchor_lang::prelude::*;
use mpl_core::{instructions::CreateCollectionV2CpiBuilder, ID as MPL_CORE_ID};

use crate::{
    constants::{COLLECTION_AUTHORITY_SEED, COLLECTION_NAME, GLOBAL_CONFIG_SEED},
    error::DoomNftProgramError,
    state::GlobalConfig,
    utils::build_collection_uri,
};

#[derive(Accounts)]
pub struct InitializeCollection<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = admin @ DoomNftProgramError::Unauthorized
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: Fresh signer for the new Core collection account.
    #[account(mut)]
    pub collection: Signer<'info>,

    /// CHECK: PDA used only as the Core collection update authority.
    #[account(
        seeds = [COLLECTION_AUTHORITY_SEED, global_config.key().as_ref()],
        bump
    )]
    pub collection_update_authority: UncheckedAccount<'info>,

    /// CHECK: Verified against the canonical Metaplex Core program id.
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process_initialize_collection(ctx: Context<InitializeCollection>) -> Result<()> {
    let global_config = &mut ctx.accounts.global_config;
    require!(
        global_config.collection == Pubkey::default(),
        DoomNftProgramError::CollectionAlreadyInitialized
    );

    let collection_uri = build_collection_uri(&global_config.base_metadata_url);
    CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .collection(&ctx.accounts.collection.to_account_info())
        .update_authority(Some(
            &ctx.accounts.collection_update_authority.to_account_info(),
        ))
        .payer(&ctx.accounts.admin.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .name(COLLECTION_NAME.to_owned())
        .uri(collection_uri)
        .invoke()?;

    global_config.collection = ctx.accounts.collection.key();
    global_config.collection_update_authority = ctx.accounts.collection_update_authority.key();

    Ok(())
}
