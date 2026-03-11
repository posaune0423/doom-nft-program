#![allow(deprecated)]

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("AavECgzCbVhHeBGAfcUgT1tYEC4N4B96E8XtF9H1fMGt");

#[program]
pub mod doom_nft_program {
    use super::*;

    pub fn initialize_global_config(
        ctx: Context<InitializeGlobalConfig>,
        base_metadata_url: String,
        upgrade_authority: Pubkey,
    ) -> Result<()> {
        instructions::initialize_global_config::process_initialize_global_config(
            ctx,
            base_metadata_url,
            upgrade_authority,
        )
    }

    pub fn initialize_collection(ctx: Context<InitializeCollection>) -> Result<()> {
        instructions::initialize_collection::process_initialize_collection(ctx)
    }

    pub fn reserve_token_id(ctx: Context<ReserveTokenId>) -> Result<()> {
        instructions::reserve_token_id::process_reserve_token_id(ctx)
    }

    pub fn mint_doom_index_nft(ctx: Context<MintDoomIndexNft>, token_id: u64) -> Result<()> {
        instructions::mint_doom_index_nft::process_mint_doom_index_nft(ctx, token_id)
    }

    pub fn update_base_metadata_url(
        ctx: Context<UpdateBaseMetadataUrl>,
        base_metadata_url: String,
    ) -> Result<()> {
        instructions::update_base_metadata_url::process_update_base_metadata_url(
            ctx,
            base_metadata_url,
        )
    }

    pub fn set_mint_paused(ctx: Context<SetMintPaused>, paused: bool) -> Result<()> {
        instructions::set_mint_paused::process_set_mint_paused(ctx, paused)
    }

    pub fn transfer_admin(ctx: Context<TransferAdmin>, new_admin: Pubkey) -> Result<()> {
        instructions::transfer_admin::process_transfer_admin(ctx, new_admin)
    }

    pub fn set_upgrade_authority(
        ctx: Context<SetUpgradeAuthority>,
        new_upgrade_authority: Pubkey,
    ) -> Result<()> {
        instructions::set_upgrade_authority::process_set_upgrade_authority(
            ctx,
            new_upgrade_authority,
        )
    }
}
