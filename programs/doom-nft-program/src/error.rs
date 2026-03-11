use anchor_lang::prelude::*;

#[error_code]
pub enum DoomNftProgramError {
    #[msg("The caller is not authorized to perform this action.")]
    Unauthorized,
    #[msg("Minting is currently paused.")]
    MintPaused,
    #[msg("The collection has not been initialized.")]
    CollectionNotInitialized,
    #[msg("The collection has already been initialized.")]
    CollectionAlreadyInitialized,
    #[msg("The reservation has already been used to mint an asset.")]
    ReservationAlreadyMinted,
    #[msg("The reservation belongs to a different user.")]
    ReservationOwnerMismatch,
    #[msg("The reservation token id does not match the instruction token id.")]
    ReservationTokenMismatch,
    #[msg("The configured collection authority does not match the derived PDA.")]
    CollectionAuthorityMismatch,
    #[msg("The base metadata url is invalid.")]
    BaseMetadataUrlInvalid,
    #[msg("The token id counter overflowed.")]
    TokenIdOverflow,
}
