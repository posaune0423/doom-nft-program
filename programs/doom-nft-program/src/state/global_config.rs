use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub upgrade_authority: Pubkey,
    pub next_token_id: u64,
    pub mint_paused: bool,
    #[max_len(256)]
    pub base_metadata_url: String,
    pub collection: Pubkey,
    pub collection_update_authority: Pubkey,
    pub bump: u8,
}
