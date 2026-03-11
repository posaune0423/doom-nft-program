use anchor_lang::prelude::*;

use crate::error::DoomNftProgramError;

pub fn validate_base_metadata_url(base_metadata_url: &str) -> Result<()> {
    require!(
        !base_metadata_url.is_empty() && !base_metadata_url.ends_with('/'),
        DoomNftProgramError::BaseMetadataUrlInvalid
    );

    Ok(())
}

pub fn build_collection_uri(base_metadata_url: &str) -> String {
    format!("{base_metadata_url}/collection.json")
}

pub fn build_asset_name(token_id: u64) -> String {
    format!("DOOM INDEX #{token_id}")
}

pub fn build_asset_uri(base_metadata_url: &str, token_id: u64) -> String {
    format!("{base_metadata_url}/{token_id}.json")
}
