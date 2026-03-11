use anchor_lang::prelude::*;

#[event]
pub struct TokenReserved {
    pub token_id: u64,
    pub reserver: Pubkey,
    pub reservation: Pubkey,
}

#[event]
pub struct AssetMinted {
    pub token_id: u64,
    pub asset: Pubkey,
    pub owner: Pubkey,
}

#[event]
pub struct BaseMetadataUrlUpdated {
    pub old_base_url: String,
    pub new_base_url: String,
}

#[event]
pub struct UpgradeAuthorityUpdated {
    pub old_upgrade_authority: Pubkey,
    pub new_upgrade_authority: Pubkey,
}
