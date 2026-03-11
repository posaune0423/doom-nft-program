use anchor_lang::prelude::*;

use crate::{
    constants::{GLOBAL_CONFIG_SEED, RESERVATION_SEED},
    error::DoomNftProgramError,
    events::TokenReserved,
    state::{GlobalConfig, MintReservation},
};

#[derive(Accounts)]
pub struct ReserveTokenId<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        init,
        payer = user,
        space = 8 + MintReservation::INIT_SPACE,
        seeds = [RESERVATION_SEED, global_config.next_token_id.to_le_bytes().as_ref()],
        bump
    )]
    pub reservation: Account<'info, MintReservation>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process_reserve_token_id(ctx: Context<ReserveTokenId>) -> Result<()> {
    let global_config = &mut ctx.accounts.global_config;
    require!(!global_config.mint_paused, DoomNftProgramError::MintPaused);

    let token_id = global_config.next_token_id;
    global_config.next_token_id = global_config
        .next_token_id
        .checked_add(1)
        .ok_or(DoomNftProgramError::TokenIdOverflow)?;

    let reservation = &mut ctx.accounts.reservation;
    reservation.token_id = token_id;
    reservation.reserver = ctx.accounts.user.key();
    reservation.minted = false;
    reservation.bump = ctx.bumps.reservation;

    emit!(TokenReserved {
        token_id,
        reserver: reservation.reserver,
        reservation: ctx.accounts.reservation.key(),
    });

    Ok(())
}
