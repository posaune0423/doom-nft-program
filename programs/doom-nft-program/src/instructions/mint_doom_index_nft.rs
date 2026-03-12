use anchor_lang::prelude::*;
use mpl_core::{instructions::CreateV2CpiBuilder, types::DataState, ID as MPL_CORE_ID};

use crate::{
    constants::{COLLECTION_AUTHORITY_SEED, GLOBAL_CONFIG_SEED, RESERVATION_SEED},
    error::DoomNftProgramError,
    events::AssetMinted,
    state::{GlobalConfig, MintReservation},
    utils::{build_asset_name, build_asset_uri},
};

#[derive(Accounts)]
#[instruction(token_id: u64)]
pub struct MintDoomIndexNft<'info> {
    #[account(
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [RESERVATION_SEED, token_id.to_le_bytes().as_ref()],
        bump = reservation.bump
    )]
    pub reservation: Account<'info, MintReservation>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Fresh signer for the new Core asset account.
    #[account(mut)]
    pub asset: Signer<'info>,

    /// CHECK: PDA signs the Core CPI as collection/asset authority.
    #[account(
        seeds = [COLLECTION_AUTHORITY_SEED, global_config.key().as_ref()],
        bump,
        address = global_config.collection_update_authority @ DoomNftProgramError::CollectionAuthorityMismatch
    )]
    pub collection_update_authority: UncheckedAccount<'info>,

    /// CHECK: Existing Core collection account. Address checked against config.
    #[account(
        mut,
        address = global_config.collection @ DoomNftProgramError::CollectionMismatch
    )]
    pub collection: UncheckedAccount<'info>,

    /// CHECK: Verified against the canonical Metaplex Core program id.
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process_mint_doom_index_nft(ctx: Context<MintDoomIndexNft>, token_id: u64) -> Result<()> {
    let global_config = &ctx.accounts.global_config;
    require!(!global_config.mint_paused, DoomNftProgramError::MintPaused);
    require!(
        global_config.collection != Pubkey::default(),
        DoomNftProgramError::CollectionNotInitialized
    );

    let reservation = &mut ctx.accounts.reservation;
    require!(
        reservation.token_id == token_id,
        DoomNftProgramError::ReservationTokenMismatch
    );
    require!(
        reservation.reserver == ctx.accounts.user.key(),
        DoomNftProgramError::ReservationOwnerMismatch
    );
    require!(
        !reservation.minted,
        DoomNftProgramError::ReservationAlreadyMinted
    );

    let name = build_asset_name(token_id);
    let uri = build_asset_uri(&global_config.base_metadata_url, token_id);
    let global_config_key = ctx.accounts.global_config.key();
    let bump = [ctx.bumps.collection_update_authority];
    let signer_seeds: &[&[u8]] = &[COLLECTION_AUTHORITY_SEED, global_config_key.as_ref(), &bump];

    CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        .collection(Some(&ctx.accounts.collection.to_account_info()))
        .authority(Some(
            &ctx.accounts.collection_update_authority.to_account_info(),
        ))
        .payer(&ctx.accounts.user.to_account_info())
        .owner(Some(&ctx.accounts.user.to_account_info()))
        .system_program(&ctx.accounts.system_program.to_account_info())
        .data_state(DataState::AccountState)
        .name(name)
        .uri(uri)
        .invoke_signed(&[signer_seeds])?;

    reservation.minted = true;

    emit!(AssetMinted {
        token_id,
        asset: ctx.accounts.asset.key(),
        owner: ctx.accounts.user.key(),
    });

    Ok(())
}
