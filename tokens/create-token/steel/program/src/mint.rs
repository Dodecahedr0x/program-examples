use create_token_api::prelude::*;
use instructions::CreateMetadataAccountV3CpiBuilder;
use mpl_token_metadata::*;
use solana_program::program::invoke;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::{initialize_mint, mint_to_checked};
use steel::*;
use sysvar::rent;
use types::{Creator, DataV2};

pub fn process_mint(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [payer, metadata, mint, token_account, token_metadata_program, associated_token_program, token_program, system_program, rent] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Check accounts validity
    payer.is_signer()?.is_writable()?;
    mint.is_empty()?.is_writable()?;
    metadata.is_empty()?.is_writable()?;
    token_account.is_empty()?.is_writable()?;
    token_metadata_program.is_program(&mpl_token_metadata::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
    token_program.is_program(&spl_token::ID)?;
    system_program.is_program(&system_program::ID)?;
    rent.is_sysvar(&rent::ID)?;

    let Mint {
        amount,
        decimals,
        symbol,
        name,
        uri,
    } = Mint::try_from_bytes(data)?;

    // Create the mint account
    initialize_mint(
        &spl_token::ID,
        mint.key,
        payer.key,
        Some(payer.key),
        *decimals as u8,
    )?;

    // Create token metadata
    CreateMetadataAccountV3CpiBuilder::new(token_metadata_program)
        .metadata(metadata)
        .mint(mint)
        .mint_authority(payer)
        .update_authority(payer, true)
        .system_program(system_program)
        .data(DataV2 {
            name: array_to_string(name),
            symbol: array_to_string(symbol),
            uri: array_to_string(uri),
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: *payer.key,
                verified: true,
                share: 100,
            }]),
            collection: None,
            uses: None,
        })
        .invoke()?;

    // Create the associated token account that will receive the tokens
    invoke(
        &create_associated_token_account(payer.key, payer.key, mint.key, &spl_token::ID),
        &[
            payer.clone(),
            token_account.clone(),
            payer.clone(),
            mint.clone(),
            system_program.clone(),
            token_program.clone(),
            associated_token_program.clone(),
        ],
    )?;

    // Mint tokens to the account
    invoke(
        &mint_to_checked(
            &spl_token::ID,
            mint.key,
            token_account.key,
            payer.key,
            &[payer.key],
            *amount,
            *decimals as u8,
        )?,
        &[
            token_program.clone(),
            mint.clone(),
            token_account.clone(),
            payer.clone(),
        ],
    )?;

    Ok(())
}
