use create_token_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{find_file, processor, read_file, BanksClient, ProgramTest};
use solana_sdk::{
    account::Account, native_token::LAMPORTS_PER_SOL, program_pack::Pack, signature::Keypair,
    signer::Signer, transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use steel::system_program;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "create_token_program",
        create_token_api::ID,
        processor!(create_token_program::process_instruction),
    );
    let metadata_program_filename = "../tests/fixtures/token_metadata.so";
    program_test.add_account(
        mpl_token_metadata::ID,
        Account {
            lamports: LAMPORTS_PER_SOL,
            data: read_file(find_file(metadata_program_filename).unwrap_or_else(|| {
                panic!("Unable to locate {metadata_program_filename}");
            })),
            owner: system_program::ID,
            executable: true,
            rent_epoch: 0,
        },
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let amount = 100000;
    let decimals = 6;
    let mint_kp = Keypair::new();

    // Submit initialize transaction.
    let ix = mint(
        payer.pubkey(),
        mint_kp.pubkey(),
        amount,
        decimals,
        "Test Token".to_string(),
        "TEST".to_string(),
        "https://solana.com".to_string(),
    );
    println!("{:?}", ix.data);
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint_kp],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    println!("{:?}", res);
    assert!(res.is_ok());

    // Verify mint was initialized.
    let mint_account = banks.get_account(mint_kp.pubkey()).await.unwrap().unwrap();
    let mint_data = spl_token::state::Mint::unpack(&mint_account.data).unwrap();
    assert_eq!(mint_account.owner, spl_token::ID);
    assert_eq!(mint_data.supply, amount);

    // Verify token account was initialized.
    let token_account = banks
        .get_account(get_associated_token_address(
            &payer.pubkey(),
            &mint_kp.pubkey(),
        ))
        .await
        .unwrap()
        .unwrap();
    let token_account_data = spl_token::state::Account::unpack(&token_account.data).unwrap();
    assert_eq!(token_account_data.owner, spl_token::ID);
    assert_eq!(token_account_data.amount, amount);
}
