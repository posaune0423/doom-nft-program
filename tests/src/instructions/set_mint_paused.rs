use solana_program_test::BanksClientError;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::test_context::{
    initialize_collection_ix, initialize_global_config_ix, mint_doom_index_nft_ix,
    process_instruction, reserve_token_id_ix, set_mint_paused_ix, start_context,
};

#[tokio::test]
async fn pause_blocks_reserve_and_mint() {
    let mut context = start_context().await;
    let payer = context.payer.pubkey();
    let upgrade_authority = Keypair::new();
    process_instruction(
        &mut context,
        initialize_global_config_ix(
            payer,
            upgrade_authority.pubkey(),
            "https://example.com/base",
        ),
        &[],
    )
    .await
    .expect("initialize global config");

    let collection = Keypair::new();
    process_instruction(
        &mut context,
        initialize_collection_ix(payer, collection.pubkey()),
        &[&collection],
    )
    .await
    .expect("initialize collection");

    process_instruction(&mut context, reserve_token_id_ix(payer, 1), &[])
        .await
        .expect("reserve token id");

    process_instruction(&mut context, set_mint_paused_ix(payer, true), &[])
        .await
        .expect("pause mint");

    let reserve_error = process_instruction(&mut context, reserve_token_id_ix(payer, 2), &[])
        .await
        .expect_err("reserve should fail when paused");
    assert!(matches!(
        reserve_error,
        BanksClientError::TransactionError(_)
    ));

    let asset = Keypair::new();
    let mint_error = process_instruction(
        &mut context,
        mint_doom_index_nft_ix(payer, 1, asset.pubkey(), collection.pubkey()),
        &[&asset],
    )
    .await
    .expect_err("mint should fail when paused");
    assert!(matches!(mint_error, BanksClientError::TransactionError(_)));
}
