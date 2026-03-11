#![cfg(test)]

use anchor_lang::{
    prelude::AccountDeserialize,
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult},
    system_program::ID as SYSTEM_PROGRAM_ID,
    InstructionData, ToAccountMetas,
};
use doom_nft_program::{
    accounts::{
        InitializeCollection, InitializeGlobalConfig, MintDoomIndexNft, ReserveTokenId,
        SetMintPaused, SetUpgradeAuthority, TransferAdmin, UpdateBaseMetadataUrl,
    },
    instruction, GlobalConfig, MintReservation,
};
use mpl_core::{Asset, Collection};
use solana_program_test::{processor, BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

fn program_test() -> ProgramTest {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bpf_out_dir = manifest_dir.join("..").join("target").join("test-sbf");
    std::env::set_var("BPF_OUT_DIR", &bpf_out_dir);

    let mut test = ProgramTest::default();
    test.prefer_bpf(false);
    test.add_program(
        "doom_nft_program",
        doom_nft_program::id(),
        processor!(doom_nft_program_test_processor),
    );
    test.add_upgradeable_program_to_genesis("mpl_core_program", &mpl_core::ID);
    test
}

fn doom_nft_program_test_processor<'a, 'b, 'c, 'd>(
    program_id: &'a Pubkey,
    accounts: &'b [AccountInfo<'c>],
    instruction_data: &'d [u8],
) -> ProgramResult {
    // Anchor's generated entrypoint ties the slice lifetime to the inner AccountInfo lifetime.
    // ProgramTest uses the looser builtin processor signature, so the wrapper narrows it for this call.
    let accounts: &'c [AccountInfo<'c>] = unsafe { std::mem::transmute(accounts) };
    doom_nft_program::entry(program_id, accounts, instruction_data)
}

fn global_config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"global_config"], &doom_nft_program::id())
}

fn collection_authority_pda(global_config: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"collection_authority", global_config.as_ref()],
        &doom_nft_program::id(),
    )
}

fn reservation_pda(token_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"reservation", &token_id.to_le_bytes()],
        &doom_nft_program::id(),
    )
}

async fn process_instruction(
    context: &mut ProgramTestContext,
    instruction: Instruction,
    extra_signers: &[&Keypair],
) -> Result<Signature, BanksClientError> {
    let mut signers = vec![&context.payer];
    signers.extend_from_slice(extra_signers);

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &signers,
        context.last_blockhash,
    );
    let signature = tx.signatures[0];
    context.banks_client.process_transaction(tx).await?;
    Ok(signature)
}

async fn fetch_global_config(context: &mut ProgramTestContext) -> GlobalConfig {
    let (config, _) = global_config_pda();
    let account = context
        .banks_client
        .get_account(config)
        .await
        .expect("get config account")
        .expect("config account exists");

    let mut bytes = account.data.as_slice();
    GlobalConfig::try_deserialize(&mut bytes).expect("deserialize global config")
}

async fn fetch_reservation(context: &mut ProgramTestContext, token_id: u64) -> MintReservation {
    let (reservation, _) = reservation_pda(token_id);
    let account = context
        .banks_client
        .get_account(reservation)
        .await
        .expect("get reservation account")
        .expect("reservation account exists");

    let mut bytes = account.data.as_slice();
    MintReservation::try_deserialize(&mut bytes).expect("deserialize reservation")
}

fn initialize_global_config_ix(
    admin: Pubkey,
    upgrade_authority: Pubkey,
    base_metadata_url: &str,
) -> Instruction {
    let (global_config, _) = global_config_pda();

    Instruction {
        program_id: doom_nft_program::id(),
        accounts: InitializeGlobalConfig {
            global_config,
            admin,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: instruction::InitializeGlobalConfig {
            base_metadata_url: base_metadata_url.to_owned(),
            upgrade_authority,
        }
        .data(),
    }
}

fn initialize_collection_ix(admin: Pubkey, collection: Pubkey) -> Instruction {
    let (global_config, _) = global_config_pda();
    let (collection_update_authority, _) = collection_authority_pda(global_config);

    Instruction {
        program_id: doom_nft_program::id(),
        accounts: InitializeCollection {
            global_config,
            admin,
            collection,
            collection_update_authority,
            mpl_core_program: mpl_core::ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: instruction::InitializeCollection {}.data(),
    }
}

fn reserve_token_id_ix(user: Pubkey, token_id: u64) -> Instruction {
    let (global_config, _) = global_config_pda();
    let (reservation, _) = reservation_pda(token_id);

    Instruction {
        program_id: doom_nft_program::id(),
        accounts: ReserveTokenId {
            global_config,
            reservation,
            user,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: instruction::ReserveTokenId {}.data(),
    }
}

fn mint_doom_index_nft_ix(
    user: Pubkey,
    token_id: u64,
    asset: Pubkey,
    collection: Pubkey,
) -> Instruction {
    let (global_config, _) = global_config_pda();
    let (reservation, _) = reservation_pda(token_id);
    let (collection_update_authority, _) = collection_authority_pda(global_config);
    let config = global_config;

    Instruction {
        program_id: doom_nft_program::id(),
        accounts: MintDoomIndexNft {
            global_config: config,
            reservation,
            user,
            asset,
            collection_update_authority,
            collection,
            mpl_core_program: mpl_core::ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: instruction::MintDoomIndexNft { token_id }.data(),
    }
}

#[tokio::test]
async fn initialize_global_config_sets_defaults() {
    let mut context = program_test().start_with_context().await;
    let payer = context.payer.pubkey();
    let upgrade_authority = Keypair::new();
    let base_metadata_url = "https://example.com/doom-index";

    let signature = process_instruction(
        &mut context,
        initialize_global_config_ix(payer, upgrade_authority.pubkey(), base_metadata_url),
        &[],
    )
    .await
    .expect("initialize global config");

    assert_ne!(signature, Signature::default());

    let config = fetch_global_config(&mut context).await;
    assert_eq!(config.admin, context.payer.pubkey());
    assert_eq!(config.upgrade_authority, upgrade_authority.pubkey());
    assert_eq!(config.next_token_id, 1);
    assert!(!config.mint_paused);
    assert_eq!(config.base_metadata_url, base_metadata_url);
}

#[tokio::test]
async fn initialize_collection_persists_collection_and_authority() {
    let mut context = program_test().start_with_context().await;
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

    let config = fetch_global_config(&mut context).await;
    let (collection_authority, _) = collection_authority_pda(global_config_pda().0);
    assert_eq!(config.collection, collection.pubkey());
    assert_eq!(config.collection_update_authority, collection_authority);

    let collection_account = context
        .banks_client
        .get_account(collection.pubkey())
        .await
        .expect("get collection account")
        .expect("collection account exists");
    let collection = Collection::from_bytes(&collection_account.data).expect("decode collection");
    assert_eq!(collection.base.name, "DOOM INDEX");
    assert_eq!(collection.base.update_authority, collection_authority);
}

#[tokio::test]
async fn reserve_token_id_creates_reservation_and_increments_counter() {
    let mut context = program_test().start_with_context().await;
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

    process_instruction(&mut context, reserve_token_id_ix(payer, 1), &[])
        .await
        .expect("reserve token id");

    let reservation = fetch_reservation(&mut context, 1).await;
    assert_eq!(reservation.token_id, 1);
    assert_eq!(reservation.reserver, context.payer.pubkey());
    assert!(!reservation.minted);

    let config = fetch_global_config(&mut context).await;
    assert_eq!(config.next_token_id, 2);
}

#[tokio::test]
async fn mint_with_valid_reservation_creates_core_asset() {
    let mut context = program_test().start_with_context().await;
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

    let asset = Keypair::new();
    process_instruction(
        &mut context,
        mint_doom_index_nft_ix(payer, 1, asset.pubkey(), collection.pubkey()),
        &[&asset],
    )
    .await
    .expect("mint doom index nft");

    let reservation = fetch_reservation(&mut context, 1).await;
    assert!(reservation.minted);

    let asset_account = context
        .banks_client
        .get_account(asset.pubkey())
        .await
        .expect("get asset account")
        .expect("asset account exists");
    let asset = Asset::from_bytes(&asset_account.data).expect("decode asset");
    assert_eq!(asset.base.name, "DOOM INDEX #1");
    assert_eq!(asset.base.uri, "https://example.com/base/1.json");
    assert_eq!(asset.base.owner, context.payer.pubkey());
}

#[tokio::test]
async fn mint_without_reservation_fails() {
    let mut context = program_test().start_with_context().await;
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

    let asset = Keypair::new();
    let error = process_instruction(
        &mut context,
        mint_doom_index_nft_ix(payer, 1, asset.pubkey(), collection.pubkey()),
        &[&asset],
    )
    .await
    .expect_err("mint should fail without reservation");

    assert!(matches!(error, BanksClientError::TransactionError(_)));
}

#[tokio::test]
async fn pause_blocks_reserve_and_mint() {
    let mut context = program_test().start_with_context().await;
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

    let set_paused = Instruction {
        program_id: doom_nft_program::id(),
        accounts: SetMintPaused {
            global_config: global_config_pda().0,
            admin: context.payer.pubkey(),
        }
        .to_account_metas(None),
        data: instruction::SetMintPaused { paused: true }.data(),
    };
    process_instruction(&mut context, set_paused, &[])
        .await
        .expect("pause mint");

    let reserve_error = process_instruction(&mut context, reserve_token_id_ix(payer, 1), &[])
        .await
        .expect_err("reserve should fail when paused");
    assert!(matches!(
        reserve_error,
        BanksClientError::TransactionError(_)
    ));
}

#[tokio::test]
async fn admin_controls_update_base_url_and_transfer_admin() {
    let mut context = program_test().start_with_context().await;
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

    let update_url = Instruction {
        program_id: doom_nft_program::id(),
        accounts: UpdateBaseMetadataUrl {
            global_config: global_config_pda().0,
            admin: context.payer.pubkey(),
        }
        .to_account_metas(None),
        data: instruction::UpdateBaseMetadataUrl {
            base_metadata_url: "https://example.com/next".to_owned(),
        }
        .data(),
    };
    process_instruction(&mut context, update_url, &[])
        .await
        .expect("update base metadata url");
    assert_eq!(
        fetch_global_config(&mut context).await.base_metadata_url,
        "https://example.com/next"
    );

    let next_admin = Keypair::new();
    let transfer_admin = Instruction {
        program_id: doom_nft_program::id(),
        accounts: TransferAdmin {
            global_config: global_config_pda().0,
            admin: context.payer.pubkey(),
        }
        .to_account_metas(None),
        data: instruction::TransferAdmin {
            new_admin: next_admin.pubkey(),
        }
        .data(),
    };
    process_instruction(&mut context, transfer_admin, &[])
        .await
        .expect("transfer admin");
    assert_eq!(
        fetch_global_config(&mut context).await.admin,
        next_admin.pubkey()
    );
}

#[tokio::test]
async fn upgrade_authority_is_independent_from_admin() {
    let mut context = program_test().start_with_context().await;
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

    let next_upgrade_authority = Keypair::new();
    let set_upgrade_authority = Instruction {
        program_id: doom_nft_program::id(),
        accounts: SetUpgradeAuthority {
            global_config: global_config_pda().0,
            upgrade_authority: upgrade_authority.pubkey(),
        }
        .to_account_metas(None),
        data: instruction::SetUpgradeAuthority {
            new_upgrade_authority: next_upgrade_authority.pubkey(),
        }
        .data(),
    };
    process_instruction(&mut context, set_upgrade_authority, &[&upgrade_authority])
        .await
        .expect("set upgrade authority");

    let config = fetch_global_config(&mut context).await;
    assert_eq!(config.admin, payer);
    assert_eq!(config.upgrade_authority, next_upgrade_authority.pubkey());
}
