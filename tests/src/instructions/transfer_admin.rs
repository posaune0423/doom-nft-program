use solana_sdk::{signature::Keypair, signer::Signer};

use crate::test_context::{
    fetch_global_config, initialize_global_config_ix, process_instruction, start_context,
    transfer_admin_ix,
};

#[tokio::test]
async fn transfer_admin_updates_admin_authority() {
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

    let next_admin = Keypair::new();
    process_instruction(
        &mut context,
        transfer_admin_ix(payer, upgrade_authority.pubkey(), next_admin.pubkey()),
        &[&upgrade_authority, &next_admin],
    )
    .await
    .expect("transfer admin");

    assert_eq!(
        fetch_global_config(&mut context).await.admin,
        next_admin.pubkey()
    );
}
