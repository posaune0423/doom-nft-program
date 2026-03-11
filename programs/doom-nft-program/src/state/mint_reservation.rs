use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MintReservation {
    pub token_id: u64,
    pub reserver: Pubkey,
    pub minted: bool,
    pub bump: u8,
}
