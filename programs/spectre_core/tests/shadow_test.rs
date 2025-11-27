use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use solana_program_test::*;
use solana_sdk::{
    signature::Signer,
    transaction::Transaction,
};

#[tokio::test]
async fn init_shadow_account() {
    let program_test = ProgramTest::new(
        "spectre_core",
        spectre_core::id(),
        None,
    );

    let mut context = program_test.start_with_context().await;

    let shadow = Keypair::new();
    let owner = Keypair::new();

    let ix = Instruction {
        program_id: spectre_core::id(),
        accounts: spectre_core::accounts::InitShadowAccount {
            shadow_account: shadow.pubkey(),
            owner: owner.pubkey(),
            payer: context.payer.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
        data: spectre_core::instruction::InitShadowAccount {}.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &shadow],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();
}
