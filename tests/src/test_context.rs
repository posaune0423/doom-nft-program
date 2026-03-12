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

pub fn program_test() -> ProgramTest {
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

pub async fn start_context() -> ProgramTestContext {
    program_test().start_with_context().await
}

fn doom_nft_program_test_processor<'a, 'b, 'c, 'd>(
    program_id: &'a Pubkey,
    accounts: &'b [AccountInfo<'c>],
    instruction_data: &'d [u8],
) -> ProgramResult {
    // SAFETY: This only narrows the outer slice lifetime from `&'b [AccountInfo<'c>]` to
    // `&'c [AccountInfo<'c>]` without changing the slice layout or the contained `AccountInfo`
    // values. `accounts` already points to `AccountInfo<'c>` elements, and callers uphold that
    // the slice lives for the full duration of `doom_nft_program::entry(program_id, accounts,
    // instruction_data)`, so this cast does not extend any inner borrow past its original
    // lifetime.
    let accounts: &'c [AccountInfo<'c>] = unsafe { std::mem::transmute(accounts) };
    doom_nft_program::entry(program_id, accounts, instruction_data)
}

pub fn global_config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"global_config"], &doom_nft_program::id())
}

pub fn collection_authority_pda(global_config: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"collection_authority", global_config.as_ref()],
        &doom_nft_program::id(),
    )
}

pub fn reservation_pda(token_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"reservation", &token_id.to_le_bytes()],
        &doom_nft_program::id(),
    )
}

pub async fn process_instruction(
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

pub async fn fetch_global_config(context: &mut ProgramTestContext) -> GlobalConfig {
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

pub async fn fetch_reservation(context: &mut ProgramTestContext, token_id: u64) -> MintReservation {
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

pub async fn fetch_collection(context: &mut ProgramTestContext, collection: Pubkey) -> Collection {
    let account = context
        .banks_client
        .get_account(collection)
        .await
        .expect("get collection account")
        .expect("collection account exists");

    *Collection::from_bytes(&account.data).expect("decode collection")
}

pub async fn fetch_asset(context: &mut ProgramTestContext, asset: Pubkey) -> Asset {
    let account = context
        .banks_client
        .get_account(asset)
        .await
        .expect("get asset account")
        .expect("asset account exists");

    *Asset::from_bytes(&account.data).expect("decode asset")
}

pub fn initialize_global_config_ix(
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

pub fn initialize_collection_ix(admin: Pubkey, collection: Pubkey) -> Instruction {
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

pub fn reserve_token_id_ix(user: Pubkey, token_id: u64) -> Instruction {
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

pub fn mint_doom_index_nft_ix(
    user: Pubkey,
    token_id: u64,
    asset: Pubkey,
    collection: Pubkey,
) -> Instruction {
    let (global_config, _) = global_config_pda();
    let (reservation, _) = reservation_pda(token_id);
    let (collection_update_authority, _) = collection_authority_pda(global_config);

    Instruction {
        program_id: doom_nft_program::id(),
        accounts: MintDoomIndexNft {
            global_config,
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

pub fn set_mint_paused_ix(admin: Pubkey, paused: bool) -> Instruction {
    Instruction {
        program_id: doom_nft_program::id(),
        accounts: SetMintPaused {
            global_config: global_config_pda().0,
            admin,
        }
        .to_account_metas(None),
        data: instruction::SetMintPaused { paused }.data(),
    }
}

pub fn update_base_metadata_url_ix(admin: Pubkey, base_metadata_url: &str) -> Instruction {
    Instruction {
        program_id: doom_nft_program::id(),
        accounts: UpdateBaseMetadataUrl {
            global_config: global_config_pda().0,
            admin,
        }
        .to_account_metas(None),
        data: instruction::UpdateBaseMetadataUrl {
            base_metadata_url: base_metadata_url.to_owned(),
        }
        .data(),
    }
}

pub fn transfer_admin_ix(
    admin: Pubkey,
    upgrade_authority: Pubkey,
    new_admin: Pubkey,
) -> Instruction {
    Instruction {
        program_id: doom_nft_program::id(),
        accounts: TransferAdmin {
            global_config: global_config_pda().0,
            admin,
            upgrade_authority,
            new_admin,
        }
        .to_account_metas(None),
        data: instruction::TransferAdmin {}.data(),
    }
}

pub fn set_upgrade_authority_ix(
    upgrade_authority: Pubkey,
    new_upgrade_authority: Pubkey,
) -> Instruction {
    Instruction {
        program_id: doom_nft_program::id(),
        accounts: SetUpgradeAuthority {
            global_config: global_config_pda().0,
            upgrade_authority,
        }
        .to_account_metas(None),
        data: instruction::SetUpgradeAuthority {
            new_upgrade_authority,
        }
        .data(),
    }
}
