use anchor_lang::prelude::*;
use url::Url;

use crate::{constants::COLLECTION_NAME, error::DoomNftProgramError};

pub fn validate_base_metadata_url(base_metadata_url: &str) -> Result<()> {
    let trimmed = base_metadata_url.trim();
    require!(
        !trimmed.is_empty()
            && trimmed == base_metadata_url
            && trimmed.len() <= 256
            && !trimmed.ends_with('/')
            && matches!(Url::parse(trimmed), Ok(url) if url.scheme() == "https"),
        DoomNftProgramError::BaseMetadataUrlInvalid
    );

    Ok(())
}

pub fn build_collection_uri(base_metadata_url: &str) -> String {
    format!("{base_metadata_url}/collection.json")
}

pub fn build_asset_name(token_id: u64) -> String {
    format!("{COLLECTION_NAME} #{token_id}")
}

pub fn build_asset_uri(base_metadata_url: &str, token_id: u64) -> String {
    format!("{base_metadata_url}/{token_id}.json")
}

#[cfg(test)]
mod tests {
    use super::validate_base_metadata_url;

    #[test]
    fn accepts_https_url_without_trailing_slash() {
        assert!(validate_base_metadata_url("https://example.com/base").is_ok());
    }

    #[test]
    fn rejects_non_https_url() {
        assert!(validate_base_metadata_url("http://example.com/base").is_err());
    }

    #[test]
    fn rejects_whitespace_padded_url() {
        assert!(validate_base_metadata_url(" https://example.com/base ").is_err());
    }

    #[test]
    fn rejects_trailing_slash() {
        assert!(validate_base_metadata_url("https://example.com/base/").is_err());
    }

    #[test]
    fn rejects_overlong_url() {
        let overlong = format!("https://example.com/{}", "a".repeat(300));
        assert!(validate_base_metadata_url(&overlong).is_err());
    }
}
